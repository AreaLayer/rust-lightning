// This file is Copyright its original authors, visible in version control history.
//
// This file is licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. You may not use this file except in
// accordance with one or both of these licenses.

use crate::common::{ConfirmedTx, FilterQueue, SyncState};
use crate::error::{InternalError, TxSyncError};

use lightning::chain::WatchedOutput;
use lightning::chain::{Confirm, Filter};
use lightning::util::logger::Logger;
use lightning::{log_debug, log_error, log_trace};

use lightning_macros::{maybe_async, maybe_await};

use bitcoin::{BlockHash, Script, Txid};

#[cfg(not(feature = "async-interface"))]
use esplora_client::blocking::BlockingClient;
#[cfg(feature = "async-interface")]
use esplora_client::r#async::AsyncClient;
use esplora_client::Builder;

use core::ops::Deref;
use std::collections::HashSet;

/// Synchronizes LDK with a given [`Esplora`] server.
///
/// Needs to be registered with a [`ChainMonitor`] via the [`Filter`] interface to be informed of
/// transactions and outputs to monitor for on-chain confirmation, unconfirmation, and
/// reconfirmation.
///
/// Note that registration via [`Filter`] needs to happen before any calls to
/// [`Watch::watch_channel`] to ensure we get notified of the items to monitor.
///
/// This uses and exposes either a blocking or async client variant dependent on whether the
/// `esplora-blocking` or the `esplora-async` feature is enabled.
///
/// [`Esplora`]: https://github.com/Blockstream/electrs
/// [`ChainMonitor`]: lightning::chain::chainmonitor::ChainMonitor
/// [`Watch::watch_channel`]: lightning::chain::Watch::watch_channel
/// [`Filter`]: lightning::chain::Filter
pub struct EsploraSyncClient<L: Deref>
where
	L::Target: Logger,
{
	sync_state: MutexType<SyncState>,
	queue: std::sync::Mutex<FilterQueue>,
	client: EsploraClientType,
	logger: L,
}

impl<L: Deref> EsploraSyncClient<L>
where
	L::Target: Logger,
{
	/// Returns a new [`EsploraSyncClient`] object.
	pub fn new(server_url: String, logger: L) -> Self {
		let builder = Builder::new(&server_url);
		#[cfg(not(feature = "async-interface"))]
		let client = builder.build_blocking();
		#[cfg(feature = "async-interface")]
		let client = builder.build_async().unwrap();

		EsploraSyncClient::from_client(client, logger)
	}

	/// Returns a new [`EsploraSyncClient`] object using the given Esplora client.
	///
	/// This is not exported to bindings users as the underlying client from BDK is not exported.
	pub fn from_client(client: EsploraClientType, logger: L) -> Self {
		let sync_state = MutexType::new(SyncState::new());
		let queue = std::sync::Mutex::new(FilterQueue::new());
		Self { sync_state, queue, client, logger }
	}

	/// Synchronizes the given `confirmables` via their [`Confirm`] interface implementations. This
	/// method should be called regularly to keep LDK up-to-date with current chain data.
	///
	/// For example, instances of [`ChannelManager`] and [`ChainMonitor`] can be informed about the
	/// newest on-chain activity related to the items previously registered via the [`Filter`]
	/// interface.
	///
	/// [`Confirm`]: lightning::chain::Confirm
	/// [`ChainMonitor`]: lightning::chain::chainmonitor::ChainMonitor
	/// [`ChannelManager`]: lightning::ln::channelmanager::ChannelManager
	/// [`Filter`]: lightning::chain::Filter
	#[maybe_async]
	pub fn sync<C: Deref>(&self, confirmables: Vec<C>) -> Result<(), TxSyncError>
	where
		C::Target: Confirm,
	{
		// This lock makes sure we're syncing once at a time.
		#[cfg(not(feature = "async-interface"))]
		let mut sync_state = self.sync_state.lock().unwrap();
		#[cfg(feature = "async-interface")]
		let mut sync_state = self.sync_state.lock().await;

		log_trace!(self.logger, "Starting transaction sync.");
		#[cfg(feature = "time")]
		let start_time = std::time::Instant::now();
		let mut num_confirmed = 0;
		let mut num_unconfirmed = 0;

		let mut tip_hash = maybe_await!(self.client.get_tip_hash())?;

		loop {
			let pending_registrations = self.queue.lock().unwrap().process_queues(&mut sync_state);
			let tip_is_new = Some(tip_hash) != sync_state.last_sync_hash;

			// We loop until any registered transactions have been processed at least once, or the
			// tip hasn't been updated during the last iteration.
			if !sync_state.pending_sync && !pending_registrations && !tip_is_new {
				// Nothing to do.
				break;
			} else {
				// Update the known tip to the newest one.
				if tip_is_new {
					// First check for any unconfirmed transactions and act on it immediately.
					match maybe_await!(self.get_unconfirmed_transactions(&confirmables)) {
						Ok(unconfirmed_txs) => {
							// Double-check the tip hash. If it changed, a reorg happened since
							// we started syncing and we need to restart last-minute.
							match maybe_await!(self.client.get_tip_hash()) {
								Ok(check_tip_hash) => {
									if check_tip_hash != tip_hash {
										tip_hash = check_tip_hash;

										log_debug!(self.logger, "Encountered inconsistency during transaction sync, restarting.");
										sync_state.pending_sync = true;
										continue;
									}
									num_unconfirmed += unconfirmed_txs.len();
									sync_state.sync_unconfirmed_transactions(
										&confirmables,
										unconfirmed_txs,
									);
								},
								Err(err) => {
									// (Semi-)permanent failure, retry later.
									log_error!(self.logger,
										"Failed during transaction sync, aborting. Synced so far: {} confirmed, {} unconfirmed.",
										num_confirmed,
										num_unconfirmed
										);
									sync_state.pending_sync = true;
									return Err(TxSyncError::from(err));
								},
							}
						},
						Err(err) => {
							// (Semi-)permanent failure, retry later.
							log_error!(self.logger,
								"Failed during transaction sync, aborting. Synced so far: {} confirmed, {} unconfirmed.",
								num_confirmed,
								num_unconfirmed
							);
							sync_state.pending_sync = true;
							return Err(TxSyncError::from(err));
						},
					}

					match maybe_await!(self.sync_best_block_updated(
						&confirmables,
						&mut sync_state,
						&tip_hash
					)) {
						Ok(()) => {},
						Err(InternalError::Inconsistency) => {
							// Immediately restart syncing when we encounter any inconsistencies.
							log_debug!(
								self.logger,
								"Encountered inconsistency during transaction sync, restarting."
							);
							sync_state.pending_sync = true;
							continue;
						},
						Err(err) => {
							// (Semi-)permanent failure, retry later.
							log_error!(self.logger,
								"Failed during transaction sync, aborting. Synced so far: {} confirmed, {} unconfirmed.",
								num_confirmed,
								num_unconfirmed
							);
							sync_state.pending_sync = true;
							return Err(TxSyncError::from(err));
						},
					}
				}

				match maybe_await!(self.get_confirmed_transactions(&sync_state)) {
					Ok(confirmed_txs) => {
						// Double-check the tip hash. If it changed, a reorg happened since
						// we started syncing and we need to restart last-minute.
						match maybe_await!(self.client.get_tip_hash()) {
							Ok(check_tip_hash) => {
								if check_tip_hash != tip_hash {
									tip_hash = check_tip_hash;

									log_debug!(self.logger,
										"Encountered inconsistency during transaction sync, restarting.");
									sync_state.pending_sync = true;
									continue;
								}
								num_confirmed += confirmed_txs.len();
								sync_state
									.sync_confirmed_transactions(&confirmables, confirmed_txs);
							},
							Err(err) => {
								// (Semi-)permanent failure, retry later.
								log_error!(self.logger,
									"Failed during transaction sync, aborting. Synced so far: {} confirmed, {} unconfirmed.",
									num_confirmed,
									num_unconfirmed
								);
								sync_state.pending_sync = true;
								return Err(TxSyncError::from(err));
							},
						}
					},
					Err(InternalError::Inconsistency) => {
						// Immediately restart syncing when we encounter any inconsistencies.
						log_debug!(
							self.logger,
							"Encountered inconsistency during transaction sync, restarting."
						);
						sync_state.pending_sync = true;
						continue;
					},
					Err(err) => {
						// (Semi-)permanent failure, retry later.
						log_error!(self.logger,
							"Failed during transaction sync, aborting. Synced so far: {} confirmed, {} unconfirmed.",
							num_confirmed,
							num_unconfirmed
						);
						sync_state.pending_sync = true;
						return Err(TxSyncError::from(err));
					},
				}
				sync_state.last_sync_hash = Some(tip_hash);
				sync_state.pending_sync = false;
			}
		}
		#[cfg(feature = "time")]
		log_debug!(
			self.logger,
			"Finished transaction sync at tip {} in {}ms: {} confirmed, {} unconfirmed.",
			tip_hash,
			start_time.elapsed().as_millis(),
			num_confirmed,
			num_unconfirmed
		);
		#[cfg(not(feature = "time"))]
		log_debug!(
			self.logger,
			"Finished transaction sync at tip {}: {} confirmed, {} unconfirmed.",
			tip_hash,
			num_confirmed,
			num_unconfirmed
		);
		Ok(())
	}

	#[maybe_async]
	fn sync_best_block_updated<C: Deref>(
		&self, confirmables: &Vec<C>, sync_state: &mut SyncState, tip_hash: &BlockHash,
	) -> Result<(), InternalError>
	where
		C::Target: Confirm,
	{
		// Inform the interface of the new block.
		let tip_header = maybe_await!(self.client.get_header_by_hash(tip_hash))?;
		let tip_status = maybe_await!(self.client.get_block_status(&tip_hash))?;
		if tip_status.in_best_chain {
			if let Some(tip_height) = tip_status.height {
				for c in confirmables {
					c.best_block_updated(&tip_header, tip_height);
				}

				// Prune any sufficiently confirmed output spends
				sync_state.prune_output_spends(tip_height);
			}
		} else {
			return Err(InternalError::Inconsistency);
		}
		Ok(())
	}

	#[maybe_async]
	fn get_confirmed_transactions(
		&self, sync_state: &SyncState,
	) -> Result<Vec<ConfirmedTx>, InternalError> {
		// First, check the confirmation status of registered transactions as well as the
		// status of dependent transactions of registered outputs.

		let mut confirmed_txs: Vec<ConfirmedTx> = Vec::new();

		for txid in &sync_state.watched_transactions {
			if confirmed_txs.iter().any(|ctx| ctx.txid == *txid) {
				continue;
			}
			if let Some(confirmed_tx) = maybe_await!(self.get_confirmed_tx(*txid, None, None))? {
				confirmed_txs.push(confirmed_tx);
			}
		}

		for (_, output) in &sync_state.watched_outputs {
			if let Some(output_status) = maybe_await!(self
				.client
				.get_output_status(&output.outpoint.txid, output.outpoint.index as u64))?
			{
				if let Some(spending_txid) = output_status.txid {
					if let Some(spending_tx_status) = output_status.status {
						if confirmed_txs.iter().any(|ctx| ctx.txid == spending_txid) {
							if spending_tx_status.confirmed {
								// Skip inserting duplicate ConfirmedTx entry
								continue;
							} else {
								log_trace!(self.logger, "Inconsistency: Detected previously-confirmed Tx {} as unconfirmed", spending_txid);
								return Err(InternalError::Inconsistency);
							}
						}

						if let Some(confirmed_tx) = maybe_await!(self.get_confirmed_tx(
							spending_txid,
							spending_tx_status.block_hash,
							spending_tx_status.block_height,
						))? {
							confirmed_txs.push(confirmed_tx);
						}
					}
				}
			}
		}

		// Sort all confirmed transactions first by block height, then by in-block
		// position, and finally feed them to the interface in order.
		confirmed_txs.sort_unstable_by(|tx1, tx2| {
			tx1.block_height.cmp(&tx2.block_height).then_with(|| tx1.pos.cmp(&tx2.pos))
		});

		Ok(confirmed_txs)
	}

	#[maybe_async]
	fn get_confirmed_tx(
		&self, txid: Txid, expected_block_hash: Option<BlockHash>, known_block_height: Option<u32>,
	) -> Result<Option<ConfirmedTx>, InternalError> {
		if let Some(merkle_block) = maybe_await!(self.client.get_merkle_block(&txid))? {
			let block_header = merkle_block.header;
			let block_hash = block_header.block_hash();
			if let Some(expected_block_hash) = expected_block_hash {
				if expected_block_hash != block_hash {
					log_trace!(
						self.logger,
						"Inconsistency: Tx {} expected in block {}, but is confirmed in {}",
						txid,
						expected_block_hash,
						block_hash
					);
					return Err(InternalError::Inconsistency);
				}
			}

			let mut matches = Vec::new();
			let mut indexes = Vec::new();
			let _ = merkle_block.txn.extract_matches(&mut matches, &mut indexes);
			if indexes.len() != 1 || matches.len() != 1 || matches[0] != txid {
				log_error!(self.logger, "Retrieved Merkle block for txid {} doesn't match expectations. This should not happen. Please verify server integrity.", txid);
				return Err(InternalError::Failed);
			}

			// unwrap() safety: len() > 0 is checked above
			let pos = *indexes.first().unwrap() as usize;
			if let Some(tx) = maybe_await!(self.client.get_tx(&txid))? {
				if tx.compute_txid() != txid {
					log_error!(self.logger, "Retrieved transaction for txid {} doesn't match expectations. This should not happen. Please verify server integrity.", txid);
					return Err(InternalError::Failed);
				}

				// Bitcoin Core's Merkle tree implementation has no way to discern between
				// internal and leaf node entries. As a consequence it is susceptible to an
				// attacker injecting additional transactions by crafting 64-byte
				// transactions matching an inner Merkle node's hash (see
				// https://web.archive.org/web/20240329003521/https://bitslog.com/2018/06/09/leaf-node-weakness-in-bitcoin-merkle-tree-design/).
				// To protect against this (highly unlikely) attack vector, we check that the
				// transaction is at least 65 bytes in length.
				if tx.total_size() == 64 {
					log_error!(
						self.logger,
						"Skipping transaction {} due to retrieving potentially invalid tx data.",
						txid
					);
					return Ok(None);
				}

				if let Some(block_height) = known_block_height {
					// We can take a shortcut here if a previous call already gave us the height.
					return Ok(Some(ConfirmedTx { tx, txid, block_header, pos, block_height }));
				}

				let block_status = maybe_await!(self.client.get_block_status(&block_hash))?;
				if let Some(block_height) = block_status.height {
					return Ok(Some(ConfirmedTx { tx, txid, block_header, pos, block_height }));
				} else {
					// If any previously-confirmed block suddenly is no longer confirmed, we found
					// an inconsistency and should start over.
					log_trace!(
						self.logger,
						"Inconsistency: Tx {} was unconfirmed during syncing.",
						txid
					);
					return Err(InternalError::Inconsistency);
				}
			}
		}
		Ok(None)
	}

	#[maybe_async]
	fn get_unconfirmed_transactions<C: Deref>(
		&self, confirmables: &Vec<C>,
	) -> Result<Vec<Txid>, InternalError>
	where
		C::Target: Confirm,
	{
		// Query the interface for relevant txids and check whether the relevant blocks are still
		// in the best chain, mark them unconfirmed otherwise
		let relevant_txids = confirmables
			.iter()
			.flat_map(|c| c.get_relevant_txids())
			.collect::<HashSet<(Txid, u32, Option<BlockHash>)>>();

		let mut unconfirmed_txs = Vec::new();

		for (txid, _conf_height, block_hash_opt) in relevant_txids {
			if let Some(block_hash) = block_hash_opt {
				let block_status = maybe_await!(self.client.get_block_status(&block_hash))?;
				if block_status.in_best_chain {
					// Skip if the block in question is still confirmed.
					continue;
				}

				unconfirmed_txs.push(txid);
			} else {
				log_error!(self.logger, "Untracked confirmation of funding transaction. Please ensure none of your channels had been created with LDK prior to version 0.0.113!");
				panic!("Untracked confirmation of funding transaction. Please ensure none of your channels had been created with LDK prior to version 0.0.113!");
			}
		}
		Ok(unconfirmed_txs)
	}

	/// Returns a reference to the underlying esplora client.
	///
	/// This is not exported to bindings users as the underlying client from BDK is not exported.
	pub fn client(&self) -> &EsploraClientType {
		&self.client
	}
}

#[cfg(feature = "async-interface")]
type MutexType<I> = futures::lock::Mutex<I>;
#[cfg(not(feature = "async-interface"))]
type MutexType<I> = std::sync::Mutex<I>;

// The underlying client type.
#[cfg(feature = "async-interface")]
type EsploraClientType = AsyncClient;
#[cfg(not(feature = "async-interface"))]
type EsploraClientType = BlockingClient;

impl<L: Deref> Filter for EsploraSyncClient<L>
where
	L::Target: Logger,
{
	fn register_tx(&self, txid: &Txid, _script_pubkey: &Script) {
		let mut locked_queue = self.queue.lock().unwrap();
		locked_queue.transactions.insert(*txid);
	}

	fn register_output(&self, output: WatchedOutput) {
		let mut locked_queue = self.queue.lock().unwrap();
		locked_queue.outputs.insert(output.outpoint.into_bitcoin_outpoint(), output);
	}
}
