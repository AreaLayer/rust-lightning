# 0.1 - Jan 15, 2025 - "Human Readable Version Numbers"

The LDK 0.1 release represents an important milestone for the LDK project. While
there are certainly many important features which are still being built, the LDK
project has come a long way, and the LDK project is happy with the quality of
the features included in this release. Thus, the project will begin doing patch
releases to fix bugs in prior versions as new features continue to ship in new
minor versions.

## API Updates
 * The `lightning-liquidity` crate has been moved into the `rust-lightning`
   git tree, enabling support for both sides of the LSPS channel open
   negotiation protocols (#3436).
 * Since its last alpha release, `lightning-liquidity` has also gained support
   for acting as an LSPS1 client (#3436).
 * This release includes support for BIP 353 Human Readable Names resolution.
   With the `dnssec` feature enabled, simply call `ChannelManager`'s
   `pay_for_offer_from_human_readable_name` with a list of lightning nodes that
   have the `dns_resolver` feature flag set (e.g. those running LDK with the
   new `lightning_dns_resolver::OMDomainResolver` set up to resolve DNS queries
   for others) and a Human Readable Name (#3346, #3179, #3283).
 * Asynchronous `ChannelMonitorUpdate` persistence (i.e. the use of
   `ChannelMonitorUpdateStatus::InProgress`) is now considered beta-quality.
   There are no known issues with it, though the likelihood of unknown issues
   is high (#3414).
 * `ChannelManager`'s `send_payment_with_route` and `send_spontaneous_payment`
   were removed. Use `send_payment` and `send_spontaneous_payment_with_retry`
   (now renamed `send_spontaneous_payment`) instead (#3430).
 * `ChannelMonitor`s no longer need to be re-persisted after deserializing the
   `ChannelManager` before beginning normal operation. As such,
   `ChannelManagerReadArgs::channel_monitors` no longer requires mutable
   references (#3322). See the Backwards Compatibility section for more info.
 * Additional information is now stored in `ChannelMonitorUpdate`s which may
   increase the average size of `ChannelMonitorUpdate`s when claiming inbound
   payments substantially. The expected maximum size of `ChannelMonitorUpdate`s
   shouldn't change materially (#3322).
 * Redundant `Event::PaymentClaimed`s will be generated more frequently on
   startup compared to previous versions.
   `Event::PaymentClaim{able,ed}::payment_id` has been added to allow for more
   robust handling of redundant events on payments with duplicate
   `PaymentHash`es (#3303, #3322).
 * `ChannelMonitorUpdate::update_id`s no longer have a magic value (of
   `u64::MAX`) for updates after a channel has been closed. They are now
   always monotonically increasing (#3355).
 * The MSRV of `lightning-transaction-sync` has been increased to rustc 1.75 due
   to its HTTP client dependencies (#3528).
 * The default `ProbabilisticScoringFeeParameters` values now recommend specific
   ratios between different penalties, and default penalties now allow for
   higher fees in order to reduce payment latency (#3495).
 * On-chain state resolution now more aggressively batches claims into single
   transactions, reducing on-chain fee costs when resolving multiple HTLCs for a
   single channel force-closure. This also reduces the on-chain reserve
   requirements for nodes using anchor channels (#3340).
 * A `MigratableKVStore` trait was added (and implemented for
   `FilesystemStore`), enabling easy migration between `KVStore`s (#3481).
 * `InvoiceRequest::amount_msats` now returns the `offer`-implied amount if a
   Bitcoin-denominated amount was set in the `offer` and no amount was set
   directly in the `invoice_request` (#3535).
 * `Event::OpenChannelRequest::push_msat` has been replaced with an enum in
   preparation for the dual-funding protocol coming in a future release (#3137).
 * `GossipVerifier` now requires a `P2PGossipSync` which holds a reference to
   the `GossipVerifier` via an `Arc` (#3432).
 * The `max_level_*` features were removed as the performance gain compared to
   doing the limiting at runtime was negligible (#3431).
 * `ChannelManager::create_bolt11_invoice` was added, deprecating the
   `lightning::ln::invoice_utils` module (#3389).
 * The `bech32` dependency has been upgraded to 0.11 across crates (#3270).
 * Support for creating BOLT 12 `invoice_request`s with a static signing key
   rather than an ephemeral one has been removed (#3264).
 * The `Router` trait no longer extends the `MessageRouter` trait, creating an
   extra argument to `ChannelManager` construction (#3326).
 * The deprecated `AvailableBalances::balance_msat` has been removed in favor of
   `ChannelMonitor::get_claimable_balances` (#3243).
 * Deprecated re-exports of `Payment{Hash,Preimage,Secret}` and `features` were
   removed (#3359).
 * `bolt11_payment::*_from_zero_amount_invoice` methods were renamed
   `*_from_variable_amount_invoice` (#3397)
 * Offer `signing_pubkey` (and related struct names) have been renamed
   `issuer_signing_pubkey` (#3218).
 * `Event::PaymentForwarded::{prev,next}_node_id` were added (#3458).
 * `Event::ChannelClosed::last_local_balance_msat` was added (#3235).
 * `RoutingMessageHandler::handle_*` now all have a `node_id` argument (#3291).
 * `lightning::util::persist::MonitorName` has been exposed (#3376).
 * `ProbabilisticScorer::live_estimated_payment_success_probability` was added
   (#3420)
 * `EcdsaChannelSigner::sign_splicing_funding_input` was added to support an
   eventual splicing feature (#3316).
 * `{Payment,Offer}Id` now support lowercase-hex formatting (#3377).

## Bug Fixes
 * Fixed a rare case where a BOLT 12 payment may be made duplicatively if the
   node crashes while processing a BOLT 12 `invoice` message (#3313).
 * Fixed a bug where a malicious sender could cause a payment `Event` to be
   generated with an `OfferId` using a payment with a lower amount than the
   corresponding BOLT 12 offer would have required. The amount in the
   `Event::Payment{Claimable,Claimed}` were still correct (#3435).
 * The `ProbabilisticScorer` model and associated default scoring parameters
   were tweaked to be more predictive of real-world results (#3368, #3495).
 * `ProbabilisticScoringFeeParameters::base_penalty_amount_multiplier_msat` no
   longer includes any pending HTLCs we already have through channels in the
   graph, avoiding over-penalizing them in comparison to other channels (#3356).
 * A `ChannelMonitor` will no longer be archived if a `MonitorEvent` containing
   a preimage for another channel is pending. This fixes an issue where a
   payment preimage needed for another channel claim is lost if events go
   un-processed for 4038 blocks (#3450).
 * `std` builds no longer send the full gossip state to peers that do not
   request it (#3390).
 * `lightning-block-sync` listeners now receive `block_connected` calls, rather
   than always receiving `filtered_block_connected` calls (#3354).
 * Fixed a bug where some transactions were broadcasted one block before their
   locktime made them candidates for inclusion in the mempool (though they would
   be automatically re-broadcasted later, #3453).
 * `ChainMonitor` now persists `ChannelMonitor`s when their `Balance` set first
   goes empty, making `ChannelMonitor` pruning more reliable on nodes that are
   only online briefly (e.g. mobile nodes, #3442).
 * BOLT 12 invoice requests now better handle intermittent internet connectivity
   (e.g. on mobile devices with app interruptions, #3010).
 * Broadcast-gossip `MessageSendEvent`s from the `ChannelMessageHandler` are now
   delivered to peers even if the peer is behind in processing relayed gossip.
   This ensures our own gossip propagates well even if we have very limited
   upload bandwidth (#3142).
 * Fixed a bug where calling `OutputSweeper::transactions_confirmed` with
   transactions from anything but the latest block may have triggered a spurious
   assertion in debug mode (#3524).

## Performance Improvements
 * LDK now verifies `channel_update` gossip messages without holding a lock,
   allowing additional parallelism during gossip sync (#3310).
 * LDK now checks if it already has certain gossip messages before verifying the
   message signatures, reducing CPU usage during gossip sync after the first
   startup (#3305).

## Node Compatibility
 * LDK now handles fields in the experimental range of BOLT 12 messages (#3237).

## Backwards Compatibility
 * Nodes with pending forwarded HTLCs or unclaimed payments cannot be
   upgraded directly from 0.0.123 or earlier to 0.1. Instead, they must
   first either resolve all pending HTLCs (including those pending
   resolution on-chain), or run 0.0.124 or 0.0.125 and resolve any HTLCs that
   were originally forwarded or received running 0.0.123 or earlier (#3355).
 * `ChannelMonitor`s not being re-persisted after deserializing the
   `ChannelManager` only applies to upgraded nodes *after* a startup with the
   old semantics completes at least once. In other words, you must deserialize
   the `ChannelManager` with an upgraded LDK, persist the `ChannelMonitor`s as
   you would on pre-0.1 versions of LDK, then continue to normal startup once,
   and for startups thereafter you can take advantage of the new semantics
   avoiding redundant persistence on startup (#3322).
 * Pending inbound payments paying a BOLT 12 `invoice` issued prior to upgrade
   to LDK 0.1 will fail. Issued BOLT 12 `offer`s remain payable (#3435).
 * `UserConfig::accept_mpp_keysend` was removed, thus the presence of pending
   inbound MPP keysend payments will prevent downgrade to LDK 0.0.115 and
   earlier (#3439).
 * Inbound payments initialized using the removed
   `ChannelManager::create_inbound_payment{,_for_hash}_legacy` API will no
   longer be accepted by LDK 0.1 (#3383).
 * Downgrading to prior versions of LDK after using `ChannelManager`'s
   `unsafe_manual_funding_transaction_generated` may cause `ChannelManager`
   deserialization to fail (#3259).
 * `ChannelDetails` serialized with LDK 0.1+ read with versions prior to 0.1
   will have `balance_msat` equal to `next_outbound_htlc_limit_msat` (#3243).

## Security
0.1 fixes a funds-theft vulnerability when paying BOLT 12 offers as well as a
funds-lockup denial-of-service issue for anchor channels.
 * When paying a BOLT 12 offer, if the recipient responds to our
   `invoice_request` with an `invoice` which had an amount different from the
   amount we intended to pay (either from the `offer` or the `amount_msats`
   passed to `ChannelManager::pay_for_offer`), LDK would pay the amount from the
   `invoice`. As a result, a malicious recipient could cause us to overpay the
   amount we intended to pay (#3535).
 * Fixed a bug where a counterparty can cause funds of ours to be locked up
   by broadcasting a revoked commitment transaction and following HTLC
   transactions in specific formats when using an anchor channel. The funds can
   be recovered by upgrading to 0.1 and replaying the counterparty's broadcasted
   transactions (using `Confirm::transactions_confirmed`) (#3537). Thanks to
   Matt Morehouse for reporting and fixing this issue.
 * Various denial-of-service issues in the formerly-alpha `lightning-liquidity`
   crate have been addressed (#3436, #3493).


# 0.0.125 - Oct 14, 2024 - "Delayed Beta Testing"

## Bug Fixes
 * On upgrade to 0.0.124, channels which were at a steady-state (i.e. for which
   the counterparty has received our latest `revoke_and_ack` message) will
   force-close upon receiving the next channel state update from our
   counterparty. When built with debug assertions a debug assertion failure will
   occur instead (#3362).
 * Listeners in a `ChainListenerSet` will now have their `block_connected`
   method called, when appropriate, rather than always having their
   `filtered_block_connected` method called with full block data (#3354).
 * Routefinding historical liquidity channel scores were made more consistent
   for channels which have very little data which has been decayed (#3362).
 * A debug assertion failure when adding nodes to the network graph after
   removal of nodes from the network graph was fixed (#3362).

In total, this release features 6 files changed, 32 insertions, 7
deletions in 5 commits since 0.0.124 from 2 authors, in alphabetical order:

 * Elias Rohrer
 * Matt Corallo


# 0.0.124 - Sep 3, 2024 - "Papercutting Feature Requests"

## API Updates
 * `rust-bitcoin` has been updated to 0.32. The new `bitcoin-io` crate is now
   used for all IO traits, irrespective of the features set on LDK crates.
   LDK crates no longer automatically force features on dependent crates where
   possible, allowing different `std`/`no-std` settings between LDK and
   rust-bitcoin crates (e.g. to disable `std` on LDK to ensure system time is
   not accessed while using `bitcoin-io`'s `std` feature). (#3063, #3239, #3249).
 * A new `lightning_types` crate was added which contains various top-level
   types. Using types from `lightning::ln::features` or
   `Payment{Hash,Preimage,Secret}` from `lightning::ln` or
   `lightning::ln::types` is now deprecated. The new crate is re-exported as
   `lightning::types` (#3234, #3253).
 * `lightning` now depends on `lightning-invoice`, rather than the other way
   around. The `lightning_invoice::payment` module has moved to
   `lightning::ln::bolt11_payment` and `lightning_invoice::utils` to
   `lightning::ln::invoice_utils` (#3234).
 * Event handlers may now return errors, causing most events to be replayed
   immediately without blocking the background processor. See documentation on
   individual `Event`s for more information on replay (#2995).
 * `ChannelDetails::balance_msat` is deprecated in favor of
   `ChainMonitor::get_claimable_balances` and the `Balance`, which now contains
   substantially more details and more accurately calculates a node-wide
   balance when `Balance::claimable_amount_satoshis` are summed (#3212, #3247).
 * `ConfirmationTarget` has two new variants - a `MaximumFeeEstimate` which can
   help to avoid spurious force-closes by ensuring we always accept feerates up
   to this value from peers as sane and a `UrgentOnChainSweep`, replacing
   `OnChainSweep` and only being used when the on-chain sweep is urgent (#3268).
 * All `ChannelMonitor`s are no longer persisted after each block connection,
   instead spreading them out over a handful of blocks to reduce load spikes.
   Note that this will increase the incidence of `ChannelMonitor`s that have
   different best blocks on startup, requiring some additional chain replay
   (but only on some `ChannelMonitor`s) on startup for `Listen` users (#2966).
 * A new format for Rapid Gossip Sync data is now supported which contains
   additional node metadata and is more extensible (#3098).
 * `ChannelManager::send_payment_with_route` is now deprecated in favor of the
   much easier to use `Channelmanager::send_payment`. Those who wish to manually
   select the route such payments go over should do so by matching the
   `payment_id` passed to `send_payment` in `Router::find_route_with_id` (#3224)
 * `lightning-transaction-sync` now takes most `Confirm`s as a generic `Deref`.
   You may need an explicit `as &(dyn Confirm)` to update existing code (#3101).
 * HTLCs will now be forwarded over any channel with a peer, rather than only
   the specific channel requested by the payment sender (#3127).
 * `Event::PaymentFailed` is now used in place of `Event::InvoiceRequestFailed`,
   holding an `Option` for the payment hash, which will be `None` when no
   invoice has been received (#3192).
 * `ChannelManager` now supports intercepting and manually paying
   `Bolt12Invoice`s, see `UserConfig::manually_handle_bolt12_invoices` (#3078).
 * `logger::Record`s now contain a `PaymentHash` (#2930).
 * `ChainMonitor` no longer uses an opaque `MonitorUpdateId`, opting to reuse
   the `ChannelMonitorUpdate::update_id` instead. Note that you no longer have
   to call `ChainMonitor::channel_monitor_updated` for
   `ChannelMonitorUpdateStatus::InProgress` updates to a monitor that were
   started without a `ChannelMonitorUpdate` (#2957).
 * `NodeAnnouncementInfo` is now an enum holding either a gossip message or
   the important fields, reducing the memory usage of `NetworkGraph` (#3072).
 * Onion message handlers now include a message context, which allows for
   blinded path authentication (#3085, #3202).
 * `ChannelManager` now supports funding with only a txid and output index, see
   `ChannelManager::unsafe_manual_funding_transaction_generated` (#3024).
 * BOLT 12 invoice requests now go out over, and accept responses over, multiple
   paths (#3087).
 * `OnionMessenger` now supports intercepting and re-forwarding onion messages
   for peers that are offline at the time of receipt when constructed with
   `new_with_offline_peer_interception` (#2973).
 * Onion message handling trait methods now generally take a `Responder` which
   can be used to create a `ResponseInstruction` to better control how responses
   are sent. The `ResponseInstruction` can also be converted to
   `MessageSendInstructions` which can be passed to `OnionMessenger`'s
   `send_onion_message` to respond asynchronously (#2907, #2996, #3263).
 * `OnionMessenger::process_pending_events_async` was added (#3060).
 * Blinded paths used for BOLT 12 `Offer`/`Refund`s are now compact when they
   expire relatively soon, making them somewhat smaller (#3011, #3080).
 * `ChannelManager::force_close_*` now take a err msg to send to peers (#2889).
 * `ChannelDetails::is_public` has been renamed to `is_announced` and
   `ChannelHandshakeConfig::announced_channel` to `announce_for_forwarding` to
   address various misconceptions about the purpose of announcement (#3257).
 * `BlindedPath`s are now split into `BlindedMessagePath`s and
   `BlindedPaymentPath`s and `advance_path_by_one` added to each (#3182).
 * `BlindedPaymentPath` now includes the `BlindedPayInfo` (#3245).
 * BOLT 12 `Offer`/`Refund` builders no longer require a description, instead
   allowing it to be set on the builder itself (#3018).
 * The `{Inbound,Outbound}HTLCState{,Details}` and `ChannelDetails` structs have
   moved to the `ln::channel_state` module (#3089).
 * `Event::OpenChannelRequest` now contains `params` and `is_announced` (#3019).
 * Peers are no longer disconnected when we force-close a channel (#3088).
 * BOLT12 `Offer` and `Refund` now implement `Readable` (#2965).
 * `RecipientOnionFields` is now included in `Event::PaymentClaimed` (#3084).
 * `ClosureReason::HolderForceClosed::broadcasted_latest_txn` was added (#3107).
 * `EcdsaChannelSigner` no longer needs to be `Writeable` and the supertrait
   `WriteableEcdsaChannelSigner` has been removed (#3059).
 * `CustomMessageHandler::peer_{,dis}connected` were added (#3105).
 * `lightning_invoice::Description::as_inner()` was added (#3203).
 * Splice-related wire messages have been updated to the latest spec (#3129).

## Bug Fixes
 * `channel_update` messages are no longer extracted from failed payments and
   applied to the network graph via `Event::PaymentPathFailed`, preventing a
   node along the path from identifying the sender of a payment (#3083).
 * In order to prevent senders from identifying the recipient of a BOLT 12 offer
   that included a blinded path, cryptographic information from blinded paths
   are now included in the invoice request verification (#3085, #3139, #3242).
 * Routes are now length-limited based on the actual onion contents rather than
   a fixed value. This ensures no routes are generated that are unpayable when
   sending HTLCs with custom TLVs, blinded paths, or metadata (#3026, #3156).
 * Unannounced LDK nodes (or ones without a network graph) will now include
   unannounced peers as introduction points in blinded paths. This addresses
   issues where test networks were not usable for BOLT 12 due to failures to
   find paths over private channels to LDK nodes. It will also enable creating
   BOLT 12 offers for nodes with no local network graph (#3132).
 * If a channel partner fails to update the feerate on a channel for some time
   and prevailing network feerates increase, LDK will now force-close
   automatically to avoid being unable to claim our funds on-chain. In order to
   reduce false-positives, it does so by comparing the channel's fee against the
   minimum `ConfirmationTarget::MinAllowed{,Non}AnchorChannelRemoteFee` we've
   seen over the past day (and do not force-close if we haven't been running for
   a full day, #3037).
 * `MonitorUpdatingPersister` did not read `ChannelMonitorUpdate`s when
   archiving a `ChannelMonitor`, causing the archived `ChannelMonitor` to be
   missing some updates. Those updates were not removed from the `KVStore` and
   monitors being archived should have no pending updates as they were persisted
   on each new block for some time before archiving (#3276).
 * `CoinSelection`s selected for commitment transactions which did not contain a
   change output no longer result in broadcasting a non-standard transaction nor
   in under-paying the target feerate (#3285). Note that such a transaction
   would fail to propagate and LDK would have continued to bump the fee until a
   different `CoinSelection` is used which did contain a change output.
 * `invoice_error`s from BOLT 12 recipients now fail payments (#3085, #3192).
 * Fixed a bug which may lead to a missing `Event::ChannelClosed` and missing
   `Error` messages for peers when a bogus funding transaction is provided for a
   batch channel open (#3029).
 * Fixed an overflow in `RawBolt11Invoice::amount_pico_btc()` reachable via
   `Bolt11Invoice::amount_milli_satoshis()`, resulting in a debug panic or bogus
   value for invoices with invalid values (#3032).
 * In incredibly rare circumstances, when using the beta asynchronous
   persistence, it is possible that the preimage for an MPP claim could fail to
   be persisted in the `ChannelMonitor` for one or more MPP parts, resulting in
   only some of the payment's value being claimed (#3120).
 * A rare race was fixed which could lead to `ChannelMonitorUpdate`s appearing
   after a full `ChannelMonitor` persistence that already contained the same
   update. This could have caused a panic on startup for users of the
   `MonitorUpdatingPersister` in rare cases after a crash (#3196).
 * Background Processor is now woken from `ChainMonitor` when new blocks appear,
   reducing the worst-case latency to see an `Event::SpendableOutputs` (#3033).
 * `OnionMessenger::get_update_future` was added, allowing it to wake the
   background processor to ensure `Event`s are processed quickly (#3194).
 * `CoinSelection`s overpaying the target feerate by more than 1% no longer
   leads to a debug assertion (#3285).

## Backwards Compatibility
 * BOLT 12 `Offer`s created in prior versions are still valid but are at risk of
   deanonymization attacks allowing identification of the recipient node (#3139)
 * BOLT 12 outbound payments in state `RecentPaymentDetails::AwaitingInvoice`
   will eventually time out after upgrade to 0.0.124 as any received invoice
   will be considered invalid (#3139).
 * BOLT 12 `Refund`s created in prior version with non-empty `Refund::paths` are
   considered invalid by `ChannelManager`. Any attempts to claim them will be
   ignored. `Refund`s without blinded paths are unaffected (#3139).
 * The format written by `impl_writeable_tlv_based_enum[_upgradable]` for tuple
   variants has changed, only impacting LDK-external use of the macros (#3160).
 * An `Event::PaymentFailed` without a payment hash will deserialize to a
   payment hash of all-0s when downgrading (#3192).
 * `Event::PaymentFailed` reasons may be mapped to similar reasons that were
   available in previous versions on downgrade (#3192).

## Performance Improvements
 * Route-finding is 11-23% faster (#3103, #3104, #2803, #3188, on an Intel Xeon
   Silver 4116 (Skylake)).
 * `lightning-block-sync` now much better avoids lock contention during parallel
   requests for block data, speeding up gossip sync from multiple peers (#3197).

## Node Compatibility
 * 0.0.123 contained a workaround for CLN v24.02 requiring the `gossip_queries`
   feature for all peers. Since an updated CLN has now shipped which does not
   require this, the workaround has been reverted (#3172).
 * LDK now supports BOLT 12 Offers without an explicit signing public key,
   allowing it to pay more compact offers generated by other nodes (#3017).
 * LDK now supports BOLT 12 Offers without descriptions when no amount is
   present (#3018).
 * A bug was fixed which might have led to LDK spuriously rejecting
   `channel_update`s that use as-yet-undefined flag bits (#3144).

In total, this release features 312 files changed, 29853 insertions, 15480
deletions in 549 commits since 0.0.123 from 26 authors, in alphabetical order:

 * Alec Chen
 * Arik Sosman
 * Duncan Dean
 * Elias Rohrer
 * Filip Gospodinov
 * G8XSU
 * Gursharan Singh
 * Harshit Verma
 * Jeffrey Czyz
 * Jiri Jakes
 * John Cantrell
 * Lalitmohansharma1
 * Matt Corallo
 * Matthew Rheaume
 * Max Fang
 * Mirebella
 * Tobin C. Harding
 * Valentine Wallace
 * Vincenzo Palazzo
 * Willem Van Lint
 * benthecarman
 * cooltexture
 * esraa
 * jbesraa
 * optout
 * shaavan


# 0.0.123 - May 08, 2024 - "BOLT12 Dust Sweeping"

## API Updates

 * To reduce risk of force-closures and improve HTLC reliability the default
   dust exposure limit has been increased to
   `MaxDustHTLCExposure::FeeRateMultiplier(10_000)`. Users with existing
   channels might want to consider using
   `ChannelManager::update_channel_config` to apply the new default (#3045).
 * `ChainMonitor::archive_fully_resolved_channel_monitors` is now provided to
   remove from memory `ChannelMonitor`s that have been fully resolved on-chain
   and are now not needed. It uses the new `Persist::archive_persisted_channel`
   to inform the storage layer that such a monitor should be archived (#2964).
 * An `OutputSweeper` is now provided which will automatically sweep
   `SpendableOutputDescriptor`s, retrying until the sweep confirms (#2825).
 * After initiating an outbound channel, a peer disconnection no longer results
   in immediate channel closure. Rather, if the peer is reconnected before the
   channel times out LDK will automatically retry opening it (#2725).
 * `PaymentPurpose` now has separate variants for BOLT12 payments, which
   include fields from the `invoice_request` as well as the `OfferId` (#2970).
 * `ChannelDetails` now includes a list of in-flight HTLCs (#2442).
 * `Event::PaymentForwarded` now includes `skimmed_fee_msat` (#2858).
 * The `hashbrown` dependency has been upgraded and the use of `ahash` as the
   no-std hash table hash function has been removed. As a consequence, LDK's
   `Hash{Map,Set}`s no longer feature several constructors when LDK is built
   with no-std; see the `util::hash_tables` module instead. On platforms that
   `getrandom` supports, setting the `possiblyrandom/getrandom` feature flag
   will ensure hash tables are resistant to HashDoS attacks, though the
   `possiblyrandom` crate should detect most common platforms (#2810, #2891).
 * `ChannelMonitor`-originated requests to the `ChannelSigner` can now fail and
   be retried using `ChannelMonitor::signer_unblocked` (#2816).
 * `SpendableOutputDescriptor::to_psbt_input` now includes the `witness_script`
   where available as well as new proprietary data which can be used to
   re-derive some spending keys from the base key (#2761, #3004).
 * `OutPoint::to_channel_id` has been removed in favor of
   `ChannelId::v1_from_funding_outpoint` in preparation for v2 channels with a
   different `ChannelId` derivation scheme (#2797).
 * `PeerManager::get_peer_node_ids` has been replaced with `list_peers` and
   `peer_by_node_id`, which provide more details (#2905).
 * `Bolt11Invoice::get_payee_pub_key` is now provided (#2909).
 * `Default[Message]Router` now take an `entropy_source` argument (#2847).
 * `ClosureReason::HTLCsTimedOut` has been separated out from
   `ClosureReason::HolderForceClosed` as it is the most common case (#2887).
 * `ClosureReason::CooperativeClosure` is now split into
   `{Counterparty,Locally}Initiated` variants (#2863).
 * `Event::ChannelPending::channel_type` is now provided (#2872).
 * `PaymentForwarded::{prev,next}_user_channel_id` are now provided (#2924).
 * Channel init messages have been refactored towards V2 channels (#2871).
 * `BumpTransactionEvent` now contains the channel and counterparty (#2873).
 * `util::scid_utils` is now public, with some trivial utilities to examine
   short channel ids (#2694).
 * `DirectedChannelInfo::{source,target}` are now public (#2870).
 * Bounds in `lightning-background-processor` were simplified by using
   `AChannelManager` (#2963).
 * The `Persist` impl for `KVStore` no longer requires `Sized`, allowing for
   the use of `dyn KVStore` as `Persist` (#2883, #2976).
 * `From<PaymentPreimage>` is now implemented for `PaymentHash` (#2918).
 * `NodeId::from_slice` is now provided (#2942).
 * `ChannelManager` deserialization may now fail with `DangerousValue` when
    LDK's persistence API was violated (#2974).

## Bug Fixes
 * Excess fees on counterparty commitment transactions are now included in the
   dust exposure calculation. This lines behavior up with some cases where
   transaction fees can be burnt, making them effectively dust exposure (#3045).
 * `Future`s used as an `std::...::Future` could grow in size unbounded if it
   was never woken. For those not using async persistence and using the async
   `lightning-background-processor`, this could cause a memory leak in the
   `ChainMonitor` (#2894).
 * Inbound channel requests that fail in
   `ChannelManager::accept_inbound_channel` would previously have stalled from
   the peer's perspective as no `error` message was sent (#2953).
 * Blinded path construction has been tuned to select paths more likely to
   succeed, improving BOLT12 payment reliability (#2911, #2912).
 * After a reorg, `lightning-transaction-sync` could have failed to follow a
   transaction that LDK needed information about (#2946).
 * `RecipientOnionFields`' `custom_tlvs` are now propagated to recipients when
   paying with blinded paths (#2975).
 * `Event::ChannelClosed` is now properly generated and peers are properly
   notified for all channels that as a part of a batch channel open fail to be
   funded (#3029).
 * In cases where user event processing is substantially delayed such that we
   complete multiple round-trips with our peers before a `PaymentSent` event is
   handled and then restart without persisting the `ChannelManager` after having
   persisted a `ChannelMonitor[Update]`, on startup we may have `Err`d trying to
   deserialize the `ChannelManager` (#3021).
 * If a peer has relatively high latency, `PeerManager` may have failed to
   establish a connection (#2993).
 * `ChannelUpdate` messages broadcasted for our own channel closures are now
   slightly more robust (#2731).
 * Deserializing malformed BOLT11 invoices may have resulted in an integer
   overflow panic in debug builds (#3032).
 * In exceedingly rare cases (no cases of this are known), LDK may have created
   an invalid serialization for a `ChannelManager` (#2998).
 * Message processing latency handling BOLT12 payments has been reduced (#2881).
 * Latency in processing `Event::SpendableOutputs` may be reduced (#3033).

## Node Compatibility
 * LDK's blinded paths were inconsistent with other implementations in several
   ways, which have been addressed (#2856, #2936, #2945).
 * LDK's messaging blinded paths now support the latest features which some
   nodes may begin relying on soon (#2961).
 * LDK's BOLT12 structs have been updated to support some last-minute changes to
   the spec (#3017, #3018).
 * CLN v24.02 requires the `gossip_queries` feature for all peers, however LDK
   by default does not set it for those not using a `P2PGossipSync` (e.g. those
   using RGS). This change was reverted in CLN v24.02.2 however for now LDK
   always sets the `gossip_queries` feature. This change is expected to be
   reverted in a future LDK release (#2959).

## Security
0.0.123 fixes a denial-of-service vulnerability which we believe to be reachable
from untrusted input when parsing invalid BOLT11 invoices containing non-ASCII
characters.
 * BOLT11 invoices with non-ASCII characters in the human-readable-part may
   cause an out-of-bounds read attempt leading to a panic (#3054). Note that all
   BOLT11 invoices containing non-ASCII characters are invalid.

In total, this release features 150 files changed, 19307 insertions, 6306
deletions in 360 commits since 0.0.121 from 17 authors, in alphabetical order:

 * Arik Sosman
 * Duncan Dean
 * Elias Rohrer
 * Evan Feenstra
 * Jeffrey Czyz
 * Keyue Bao
 * Matt Corallo
 * Orbital
 * Sergi Delgado Segura
 * Valentine Wallace
 * Willem Van Lint
 * Wilmer Paulino
 * benthecarman
 * jbesraa
 * olegkubrakov
 * optout
 * shaavan


# 0.0.122 - Apr 09, 2024 - "That Which Is Untested Is Broken"

## Bug Fixes
 * `Route` objects did not successfully round-trip through de/serialization
   since LDK 0.0.117, which has now been fixed (#2897).
 * Correct deserialization of unknown future enum variants. This ensures
   downgrades from future versions of LDK do not result in read failures or
   corrupt reads in cases where enums are written (#2969).
 * When hitting lnd bug 6039, our workaround previously resulted in
   `ChannelManager` persistences on every round-trip with our peer. These
   useless persistences are now skipped (#2937).

In total, this release features 4 files changed, 99 insertions, 55
deletions in 6 commits from 1 author, in alphabetical order:
 * Matt Corallo


# 0.0.121 - Jan 22, 2024 - "Unwraps are Bad"

## Bug Fixes
 * Fix a deadlock when calling `batch_funding_transaction_generated` with
   invalid input (#2841).

## Security
0.0.121 fixes a denial-of-service vulnerability which is reachable from
untrusted input from peers in rare cases if we have a public channel or in
common cases if `P2PGossipSync` is used.
 * A peer that failed to complete its handshake would cause a reachable
   `unwrap` in LDK since 0.0.119 when LDK attempts to broadcast gossip to all
   peers (#2842).

In total, this release features 4 files changed, 52 insertions, 10
deletions in 4 commits from 2 authors, in alphabetical order:
 * Jeffrey Czyz
 * Matt Corallo


# 0.0.120 - Jan 17, 2024 - "Unblinded Fuzzers"

## API Updates
 * The `PeerManager` bound on `UtxoLookup` was removed entirely. This enables
   use of `UtxoLookup` in cases broken in 0.0.119 by #2773 (#2822).
 * LDK now exposes and fully implements the route blinding feature (#2812).
 * The `lightning-transaction-sync` crate no longer relies on system time
   without the `time` feature (#2799, #2817).
 * `lightning::onion_message`'s module layout has changed (#2821).
 * `Event::ChannelClosed` now includes the `channel_funding_txo` (#2800).
 * `CandidateRouteHop` variants were destructured into individual structs,
   hiding some fields which were not generally consumable (#2802).

## Bug Fixes
 * Fixed a rare issue where `lightning-net-tokio` may not fully flush its send
   buffer, leading to connection hangs (#2832).
 * Fixed a panic which may occur when connecting to a peer if we opened a second
   channel with that peer while they were disconnected (#2808).
 * Retries for a payment which previously failed in a blinded path will now
   always use an alternative blinded path (#2818).
 * `Feature`'s `Eq` and `Hash` implementation now ignore dummy bytes (#2808).
 * Some missing `DiscardFunding` or `ChannelClosed` events are now generated in
   rare funding-related failures (#2809).
 * Fixed a privacy issue in blinded path generation where the real
   `cltv_expiry_delta` would be exposed to senders (#2831).

## Security
0.0.120 fixes a denial-of-service vulnerability which is reachable from
untrusted input from peers if the `UserConfig::manually_accept_inbound_channels`
option is enabled.
 * A peer that sent an `open_channel` message with the `channel_type` field
   unfilled would trigger a reachable `unwrap` since LDK 0.0.117 (#2808).
 * In protocols where a funding output is shared with our counterparty before
   it is given to LDK, a malicious peer could have caused a reachable panic
   by reusing the same funding info in (#2809).

In total, this release features 67 files changed, 3016 insertions, 2473
deletions in 79 commits from 9 authors, in alphabetical order:
 * Elias Rohrer
 * Jeffrey Czyz
 * José A.P
 * Matt Corallo
 * Tibo-lg
 * Valentine Wallace
 * benthecarman
 * optout
 * shuoer86


# 0.0.119 - Dec 15, 2023 - "Spring Cleaning for Christmas"

## API Updates
 * The LDK crate ecosystem MSRV has been increased to 1.63 (#2681).
 * The `bitcoin` dependency has been updated to version 0.30 (#2740).
 * `lightning-invoice::payment::*` have been replaced with parameter generation
   via `payment_parameters_from[_zero_amount]_invoice` (#2727).
 * `{CoinSelection,Wallet}Source::sign_tx` are now `sign_psbt`, providing more
   information, incl spent outputs, about the transaction being signed (#2775).
 * Logger `Record`s now include `channel_id` and `peer_id` fields. These are
   opportunistically filled in when a log record is specific to a given channel
   and/or peer, and may occasionally be spuriously empty (#2314).
 * When handling send or reply onion messages (e.g. for BOLT12 payments), a new
   `Event::ConnectionNeeded` may be raised, indicating a direct connection
   should be made to a payee or an introduction point. This event is expected to
   be removed once onion message forwarding is widespread in the network (#2723)
 * Scoring data decay now happens via `ScoreUpDate::time_passed`, called from
   `lightning-background-processor`. `process_events_async` now takes a new
   time-fetch function, and `ScoreUpDate` methods now take the current time as a
   `Duration` argument. This avoids fetching time during pathfinding (#2656).
 * Receiving payments to multi-hop blinded paths is now supported (#2688).
 * `MessageRouter` and `Router` now feature methods to generate blinded paths to
   the local node for incoming messages and payments. `Router` now extends
   `MessageRouter`, and both are used in `ChannelManager` when processing or
   creating BOLT12 structures to generate multi-hop blinded paths (#1781).
 * `lightning-transaction-sync` now supports Electrum-based sync (#2685).
 * `Confirm::get_relevant_txids` now returns the height at which a transaction
   was confirmed. This can be used to assist in reorg detection (#2685).
 * `ConfirmationTarget::MaxAllowedNonAnchorChannelRemoteFee` has been removed.
   Non-anchor channel feerates are bounded indirectly through
   `ChannelConfig::max_dust_htlc_exposure` (#2696).
 * `lightning-invoice` `Description`s now rely on `UntrustedString` for
   sanitization (#2730).
 * `ScoreLookUp::channel_penalty_msat` now uses `CandidateRouteHop` (#2551).
 * The `EcdsaChannelSigner` trait was moved to `lightning::sign::ecdsa` (#2512).
 * `SignerProvider::get_destination_script` now takes `channel_keys_id` (#2744)
 * `SpendableOutputDescriptor::StaticOutput` now has `channel_keys_id` (#2749).
 * `EcdsaChannelSigner::sign_counterparty_commitment` now takes HTLC preimages
   for both inbound and outbound HTLCs (#2753).
 * `ClaimedHTLC` now includes a `counterparty_skimmed_fee_msat` field (#2715).
 * `peel_payment_onion` was added to decode an encrypted onion for a payment
   without receiving an HTLC. This allows for stateless verification of if a
   theoretical payment would be accepted prior to receipt (#2700).
 * `create_payment_onion` was added to construct an encrypted onion for a
   payment path without sending an HTLC immediately (#2677).
 * Various keys used in channels are now wrapped to provide type-safety for
   specific usages of the keys (#2675).
 * `TaggedHash` now includes the raw `tag` and `merkle_root` (#2687).
 * `Offer::is_expired_no_std` was added (#2689).
 * `PaymentPurpose::preimage()` was added (#2768).
 * `temporary_channel_id` can now be specified in `create_channel` (#2699).
 * Wire definitions for splicing messages were added (#2544).
 * Various `lightning-invoice` structs now impl `Display`, now have pub fields,
   or impl `From` (#2730).
 * The `Hash` trait is now implemented for more structs, incl P2P msgs (#2716).

## Performance Improvements
 * Memory allocations (though not memory usage) have been substantially reduced,
   meaning less overhead and hopefully less memory fragmentation (#2708, #2779).

## Bug Fixes
 * Since 0.0.117, calling `close_channel*` on a channel which has not yet been
   funded would previously result in an infinite loop and hang (#2760).
 * Since 0.0.116, sending payments requiring data in the onion for the recipient
   which was too large for the onion may have caused corruption which resulted
   in payment failure (#2752).
 * Cooperative channel closure on channels with remaining output HTLCs may have
   spuriously force-closed (#2529).
 * In LDK versions 0.0.116 through 0.0.118, in rare cases where skimmed fees are
   present on shutdown the `ChannelManager` may fail to deserialize (#2735).
 * `ChannelConfig::max_dust_exposure` values which, converted to absolute fees,
   exceeded 2^63 - 1 would result in an overflow and could lead to spurious
   payment failures or channel closures (#2722).
 * In cases where LDK is operating with provably-stale state, it panics to
   avoid funds loss. This may not have happened in cases where LDK was behind
   only exactly one state, leading instead to a revoked broadcast and funds
   loss (#2721).
 * Fixed a bug where decoding `Txid`s from Bitcoin Core JSON-RPC responses using
   `lightning-block-sync` would not properly byte-swap the hash. Note that LDK
   does not use this API internally (#2796).

## Backwards Compatibility
 * `ChannelManager`s written with LDK 0.0.119 are no longer readable by versions
   of LDK prior to 0.0.113. Users wishing to downgrade to LDK 0.0.112 or before
   can read an 0.0.119-serialized `ChannelManager` with a version of LDK from
   0.0.113 to 0.0.118, re-serialize it, and then downgrade (#2708).
 * Nodes that upgrade to 0.0.119 and subsequently downgrade after receiving a
   payment to a blinded path may leak recipient information if one or more of
   those HTLCs later fails (#2688).
 * Similarly, forwarding a blinded HTLC and subsequently downgrading to an LDK
   version prior to 0.0.119 may result in leaking the path information to the
   payment sender (#2540).

In total, this release features 148 files changed, 13780 insertions, 6279
deletions in 280 commits from 22 authors, in alphabetical order:
 * Arik Sosman
 * Chris Waterson
 * Elias Rohrer
 * Evan Feenstra
 * Gursharan Singh
 * Jeffrey Czyz
 * John Cantrell
 * Lalitmohansharma1
 * Matt Corallo
 * Matthew Rheaume
 * Orbital
 * Rachel Malonson
 * Valentine Wallace
 * Willem Van Lint
 * Wilmer Paulino
 * alexanderwiederin
 * benthecarman
 * henghonglee
 * jbesraa
 * olegkubrakov
 * optout
 * shaavan


# 0.0.118 - Oct 23, 2023 - "Just the Twelve Sinks"

## API Updates
 * BOLT12 sending and receiving is now supported as an alpha feature. You may
   run into unexpected issues and will need to have a direct connection with
   the offer's blinded path introduction points as messages are not yet routed.
   We are seeking feedback from early testers (#2578, #2039).
 * `ConfirmationTarget` has been rewritten to provide information about the
   specific use LDK needs the feerate estimate for, rather than the generic
   low-, medium-, and high-priority estimates. This allows LDK users to more
   accurately target their feerate estimates (#2660). For those wishing to
   retain their existing behavior, see the table below for conversion.
 * `ChainHash` is now used in place of `BlockHash` where it represents the
   genesis block (#2662).
 * `lightning-invoice` payment utilities now take a `Deref` to
   `AChannelManager` (#2652).
 * `peel_onion` is provided to statelessly decode an `OnionMessage` (#2599).
 * `ToSocketAddrs` + `Display` are now impl'd for `SocketAddress` (#2636, #2670)
 * `Display` is now implemented for `OutPoint` (#2649).
 * `Features::from_be_bytes` is now provided (#2640).

For those moving to the new `ConfirmationTarget`, the new variants in terms of
the old mempool/low/medium/high priorities are as follows:
 * `OnChainSweep` = `HighPriority`
 * `MaxAllowedNonAnchorChannelRemoteFee` = `max(25 * 250, HighPriority * 10)`
 * `MinAllowedAnchorChannelRemoteFee` = `MempoolMinimum`
 * `MinAllowedNonAnchorChannelRemoteFee` = `Background - 250`
 * `AnchorChannelFee` = `Background`
 * `NonAnchorChannelFee` = `Normal`
 * `ChannelCloseMinimum` = `Background`

## Bug Fixes
 * Calling `ChannelManager::close_channel[_with_feerate_and_script]` on a
   channel which did not exist would immediately hang holding several key
   `ChannelManager`-internal locks (#2657).
 * Channel information updates received from a failing HTLC are no longer
   applied to our `NetworkGraph`. This prevents a node which we attempted to
   route a payment through from being able to learn the sender of the payment.
   In some rare cases, this may result in marginally reduced payment success
   rates (#2666).
 * Anchor outputs are now properly considered when calculating the amount
   available to send in HTLCs. This can prevent force-closes in anchor channels
   when sending payments which overflow the available balance (#2674).
 * A peer that sends an `update_fulfill_htlc` message for a forwarded HTLC,
   then reconnects prior to sending a `commitment_signed` (thus retransmitting
   their `update_fulfill_htlc`) may result in the channel stalling and being
   unable to make progress (#2661).
 * In exceedingly rare circumstances, messages intended to be sent to a peer
   prior to reconnection can be sent after reconnection. This could result in
   undefined channel state and force-closes (#2663).

## Backwards Compatibility

 * Creating a blinded path to receive a payment then downgrading to LDK prior to
   0.0.117 may result in failure to receive the payment (#2413).
 * Calling `ChannelManager::pay_for_offer` or
   `ChannelManager::create_refund_builder` may prevent downgrading to LDK prior
   to 0.0.118 until the payment times out and has been removed (#2039).

## Node Compatibility
 * LDK now sends a bogus `channel_reestablish` message to peers when they ask to
   resume an unknown channel. This should cause LND nodes to force-close and
   broadcast the latest channel state to the chain. In order to trigger this
   when we wish to force-close a channel, LDK now disconnects immediately after
   sending a channel-closing `error` message. This should result in cooperative
   peers also working to confirm the latest commitment transaction when we wish
   to force-close (#2658).

## Security
0.0.118 expands mitigations against transaction cycling attacks to non-anchor
channels, though note that no mitigations which exist today are considered robust
to prevent the class of attacks.
 * In order to mitigate against transaction cycling attacks, non-anchor HTLC
   transactions are now properly re-signed before broadcasting (#2667).

In total, this release features 61 files changed, 3470 insertions, 1503
deletions in 85 commits from 12 authors, in alphabetical order:
 * Antonio Yang
 * Elias Rohrer
 * Evan Feenstra
 * Fedeparma74
 * Gursharan Singh
 * Jeffrey Czyz
 * Matt Corallo
 * Sergi Delgado Segura
 * Vladimir Fomene
 * Wilmer Paulino
 * benthecarman
 * slanesuke


# 0.0.117 - Oct 3, 2023 - "Everything but the Twelve Sinks"

## API Updates
 * `ProbabilisticScorer`'s internal models have been substantially improved,
   including better decaying (#1789), a more granular historical channel
   liquidity tracker (#2176) and a now-default option to make our estimate for a
   channel's current liquidity nonlinear in the channel's capacity (#2547). In
   total, these changes should result in improved payment success rates at the
   cost of slightly worse routefinding performance.
 * Support for custom TLVs for recipients of HTLCs has been added (#2308).
 * Support for generating transactions for third-party watchtowers has been
   added to `ChannelMonitor/Update`s (#2337).
 * `KVStorePersister` has been replaced with a more generic and featureful
   `KVStore` interface (#2472).
 * A new `MonitorUpdatingPersister` is provided which wraps a `KVStore` and
   implements `Persist` by writing differential updates rather than full
   `ChannelMonitor`s (#2359).
 * Batch funding of outbound channels is now supported using the new
   `ChannelManager::batch_funding_transaction_generated` method (#2486).
 * `ChannelManager::send_preflight_probes` has been added to probe a payment's
   potential paths while a user is providing approval for a payment (#2534).
 * Fully asynchronous `ChannelMonitor` updating is available as an alpha
   preview. There remain a few known but incredibly rare race conditions which
   may lead to loss of funds (#2112, #2169, #2562).
 * `ChannelMonitorUpdateStatus::PermanentFailure` has been removed in favor of a
   new `ChannelMonitorUpdateStatus::UnrecoverableError`. The new variant panics
   on use, rather than force-closing a channel in an unsafe manner, which the
   previous variant did (#2562). Rather than panicking with the new variant,
   users may wish to use the new asynchronous `ChannelMonitor` updating using
   `ChannelMonitorUpdateStatus::InProgress`.
 * `RouteParameters::max_total_routing_fee_msat` was added to limit the fees
   paid when routing, defaulting to 1% + 50sats when using the new
   `from_payment_params_and_value` constructor (#2417, #2603, #2604).
 * Implementations of `UtxoSource` are now provided in `lightning-block-sync`.
   Those running with a full node should use this to validate gossip (#2248).
 * `LockableScore` now supports read locking for parallel routefinding (#2197).
 * `ChannelMonitor::get_spendable_outputs` was added to allow for re-generation
   of `SpendableOutputDescriptor`s for a channel after they were provided via
   `Event::SpendableOutputs` (#2609, #2624).
 * `[u8; 32]` has been replaced with a `ChannelId` newtype for chan ids (#2485).
 * `NetAddress` was renamed `SocketAddress` (#2549) and `FromStr` impl'd (#2134)
 * For `no-std` users, `parse_onion_address` was added which creates a
   `NetAddress` from a "...onion" string and port (#2134, #2633).
 * HTLC information is now provided in `Event::PaymentClaimed::htlcs` (#2478).
 * The success probability used in historical penalties when scoring is now
   available via `historical_estimated_payment_success_probability` (#2466).
 * `RecentPaymentDetails::*::payment_id` has been added (#2567).
 * `Route` now contains a `RouteParameters` rather than a `PaymentParameters`,
   tracking the original arguments passed to routefinding (#2555).
 * `Balance::*::claimable_amount_satoshis` was renamed `amount_satoshis` (#2460)
 * `*Features::set_*_feature_bit` have been added for non-custom flags (#2522).
 * `channel_id` was added to `SpendableOutputs` events (#2511).
 * `counterparty_node_id` and `channel_capacity_sats` were added to
   `ChannelClosed` events (#2387).
 * `ChannelMonitor` now implements `Clone` for `Clone`able signers (#2448).
 * `create_onion_message` was added to build an onion message (#2583, #2595).
 * `HTLCDescriptor` now implements `Writeable`/`Readable` (#2571).
 * `SpendableOutputDescriptor` now implements `Hash` (#2602).
 * `MonitorUpdateId` now implements `Debug` (#2594).
 * `Payment{Hash,Id,Preimage}` now implement `Display` (#2492).
 * `NodeSigner::sign_bolt12_invoice{,request}` were added for future use (#2432)

## Backwards Compatibility
 * Users migrating to the new `KVStore` can use a concatentation of
   `[{primary_namespace}/[{secondary_namespace}/]]{key}` to build a key
   compatible with the previous `KVStorePersister` interface (#2472).
 * Downgrading after receipt of a payment with custom HTLC TLVs may result in
   unintentionally accepting payments with TLVs you do not understand (#2308).
 * `Route` objects (including pending payments) written by LDK versions prior
   to 0.0.117 won't be retryable after being deserialized by LDK 0.0.117 or
   above (#2555).
 * Users of the `MonitorUpdatingPersister` can upgrade seamlessly from the
   default `KVStore` `Persist` implementation, however the stored
   `ChannelMonitor`s are deliberately unreadable by the default `Persist`. This
   ensures the correct downgrade procedure is followed, which is: (#2359)
   * First, make a backup copy of all channel state,
   * then ensure all `ChannelMonitorUpdate`s stored are fully applied to the
     relevant `ChannelMonitor`,
   * finally, write each full `ChannelMonitor` using your new `Persist` impl.

## Bug Fixes
 * Anchor channels which were closed by a counterparty broadcasting its
   commitment transaction (i.e. force-closing) would previously not generate a
   `SpendableOutputs` event for our `to_remote` (i.e. non-HTLC-encumbered)
   balance. Those with such balances available should fetch the missing
   `SpendableOutputDescriptor`s using the new
   `ChannelMonitor::get_spendable_outputs` method (#2605).
 * Anchor channels may result in spurious or missing `Balance` entries for HTLC
   balances (#2610).
 * `ChannelManager::send_spontaneous_payment_with_retry` spuriously did not
   provide the recipient with enough information to claim the payment, leading
   to all spontaneous payments failing (#2475).
   `send_spontaneous_payment_with_route` was unaffected.
 * The `keysend` feature on node announcements was spuriously un-set in 0.0.112
   and has been re-enabled (#2465).
 * Fixed several races which could lead to deadlock when force-closing a channel
   (#2597). These races have not been seen in production.
 * The `ChannelManager` is persisted substantially less when it has not changed,
   leading to substantially less I/O traffic for it (#2521, #2617).
 * Passing new block data to `ChainMonitor` no longer results in all other
   monitor operations being blocked until it completes (#2528).
 * When retrying payments, any excess amount sent to the recipient in order to
   meet an `htlc_minimum` constraint on the path is now no longer included in
   the amount we send in the retry (#2575).
 * Several edge cases in route-finding around HTLC minimums were fixed which
   could have caused invalid routes or panics when built with debug assertions
   (#2570, #2575).
 * Several edge cases in route-finding around HTLC minimums and route hints
   were fixed which would spuriously result in no route found (#2575, #2604).
 * The `user_channel_id` passed to `SignerProvider::generate_channel_keys_id`
   for inbound channels is now correctly using the one passed to
   `ChannelManager::accept_inbound_channel` rather than a default value (#2428).
 * Users of `impl_writeable_tlv_based!` no longer have use requirements (#2506).
 * No longer force-close channels when counterparties send a `channel_update`
   with a bogus `htlc_minimum_msat`, which LND users can manually build (#2611).

## Node Compatibility
 * LDK now ignores `error` messages generated by LND in response to a
   `shutdown` message, avoiding force-closes due to LND bug 6039. This may
   lead to non-trivial bandwidth usage with LND peers exhibiting this bug
   during the cooperative shutdown process (#2507).

## Security
0.0.117 fixes several loss-of-funds vulnerabilities in anchor output channels,
support for which was added in 0.0.116, in reorg handling, and when accepting
channel(s) from counterparties which are miners.
 * When a counterparty broadcasts their latest commitment transaction for a
   channel with anchor outputs, we'd previously fail to build claiming
   transactions against any HTLC outputs in that transaction. This could lead
   to loss of funds if the counterparty is able to eventually claim the HTLC
   after a timeout (#2606).
 * Anchor channels HTLC claims on-chain previously spent the entire value of any
   HTLCs as fee, which has now been fixed (#2587).
 * If a channel is closed via an on-chain commitment transaction confirmation
   with a pending outbound HTLC in the commitment transaction, followed by a
   reorg which replaces the confirmed commitment transaction with a different
   (but non-revoked) commitment transaction, all before we learn the payment
   preimage for this HTLC, we may previously have not generated a proper
   claiming transaction for the HTLC's value (#2623).
 * 0.0.117 now correctly handles channels for which our counterparty funded the
   channel with a coinbase transaction. As such transactions are not spendable
   until they've reached 100 confirmations, this could have resulted in
   accepting HTLC(s) which are not enforcible on-chain (#1924).

In total, this release features 121 files changed, 20477 insertions, 8184
deletions in 381 commits from 27 authors, in alphabetical order:
 * Alec Chen
 * Allan Douglas R. de Oliveira
 * Antonio Yang
 * Arik Sosman
 * Chris Waterson
 * David Caseria
 * DhananjayPurohit
 * Dom Zippilli
 * Duncan Dean
 * Elias Rohrer
 * Erik De Smedt
 * Evan Feenstra
 * Gabor Szabo
 * Gursharan Singh
 * Jeffrey Czyz
 * Joseph Goulden
 * Lalitmohansharma1
 * Matt Corallo
 * Rachel Malonson
 * Sergi Delgado Segura
 * Valentine Wallace
 * Vladimir Fomene
 * Willem Van Lint
 * Wilmer Paulino
 * benthecarman
 * jbesraa
 * optout


# 0.0.116 - Jul 21, 2023 - "Anchoring the Roadmap"

## API Updates

 * Support for zero-HTLC-fee anchor output channels has been added and is now
   considered beta (#2367). Users who set
   `ChannelHandshakeConfig::negotiate_anchors_zero_fee_htlc_tx` should be
   prepared to handle the new `Event::BumpTransaction`, e.g. via the
   `BumpTransactionEventHandler` (#2089). Note that in order to do so you must
   ensure you always have a reserve of available unspent on-chain funds to use
   for CPFP. LDK currently makes no attempt to ensure this for you.
 * Users who set `ChannelHandshakeConfig::negotiate_anchors_zero_fee_htlc_tx`
   and wish to accept inbound anchor-based channels must do so manually by
   setting `UserConfig::manually_accept_inbound_channels` (#2368).
 * Support forwarding and accepting HTLCs with a reduced amount has been added,
   to support LSPs skimming a fee on the penultimate hop (#2319).
 * BOLT11 and BOLT12 Invoice and related types have been renamed to include a
   BOLTNN prefix, ensuring uniqueness in `lightning{,-invoice}` crates (#2416).
 * `Score`rs now have an associated type which represents a parameter passed
   when calculating penalties. This allows for the same `Score`r to be used with
   different penalty calculation parameters (#2237).
 * `DefaultRouter` is no longer restrained to a `Mutex`-wrapped `Score`,
   allowing it to be used in `no-std` builds (#2383).
 * `CustomMessageHandler::provided_{node,init}_features` and various custom
   feature bit methods on `*Features` were added (#2204).
 * Keysend/push payments using MPP are now supported when receiving if
   `UserConfig::accept_mpp_keysend` is set and when sending if specified in the
   `PaymentParameters`. Note that not all recipients support this (#2156).
 * A new `ConfirmationTarget::MempoolMinimum` has been added (#2415).
 * `SpendableOutputDescriptor::to_psbt_input` was added (#2286).
 * `ChannelManager::update_partial_channel_config` was added (#2330).
 * `ChannelDetails::channel_shutdown_state` was added (#2347).
 * The shutdown script can now be provided at shutdown time via
   `ChannelManager::close_channel_with_feerate_and_script` (#2219).
 * `BroadcasterInterface` now takes multiple transactions at once. While not
   available today, in the future single calls should be passed to a full node
   via a single batch/package transaction acceptance API (#2272).
 * `Balance::claimable_amount_satoshis` was added (#2333).
 * `payment_{hash,preimage}` have been added to some `Balance` variants (#2217).
 * The `lightning::chain::keysinterface` is now `lightning::sign` (#2246).
 * Routing to a blinded path has been implemented, though sending to such a
   route is not yet supported in `ChannelManager` (#2120).
 * `OffersMessageHandler` was added for offers-related onion messages (#2294).
 * The `CustomMessageHandler` parameter to `PeerManager` has moved to
   `MessageHandler` from `PeerManager::new` explicitly (#2249).
 * Various P2P messages for dual funding channel establishment have been added,
   though handling for them is not yet in `ChannelManager` (#1794)
 * Script-fetching methods in `sign` interfaces can now return errors, see docs
   for the implications of failing (#2213).
 * The `data_loss_protect` option is now required when reading
   `channel_reestablish` messages, as many others have done (#2253).
 * `InFlightHtlcs::add_inflight_htlc` has been added (#2042).
 * The `init` message `networks` field is now written and checked (#2329).
 * `PeerManager` generics have been simplified with the introduction of the
   `APeerManager` trait (#2249).
 * `ParitalOrd` and `Ord` are now implemented for `Invoice` (#2279).
 * `ParitalEq` and `Debug` are now implemented for `InMemorySigner` (#2328).
 * `ParitalEq` and `Eq` are now implemented for `PaymentError` (#2316).
 * `NetworkGraph::update_channel_from_announcement_no_lookup` was added (#2222).
 * `lightning::routing::gossip::verify_{channel,node}_announcement` was added
   (#2307).

## Backwards Compatibility
 * `PaymentParameters` written with blinded path info using LDK 0.0.115 will not
   be readable in LDK 0.0.116, and vice versa.
 * Forwarding less than `Event::HTLCIntercepted::expected_outbound_amount_msat`
   in `ChannelManager::forward_intercepted_htlc` may prevent the
   `ChannelManager` from being read by LDK prior to 0.0.116 (#2319)
 * Setting `ChannelConfig::accept_underpaying_htlcs` may prevent the
   `ChannelManager` from being read by LDK prior to 0.0.116 and un-setting the
   parameter between restarts may lead to payment failures (#2319).
 * `ChannelManager::create_inbound_payment{,_for_hash}_legacy` has been removed,
   removing the ability to create inbound payments which are claimable after
   downgrade to LDK 0.0.103 and prior. In the future handling such payments will
   also be removed (#2351).
 * Some fields required by LDK 0.0.103 and earlier are no longer written, thus
   deserializing objects written by 0.0.116 with 0.0.103 may now fail (#2351).

## Bug Fixes
 * `ChannelDetails::next_outbound_htlc_limit_msat` was made substantially more
   accurate and a corresponding `next_outbound_htlc_minimum_msat` was added.
   This resolves issues where unpayable routes were generated due to
   overestimation of the amount which is payable over one of our channels as
   the first hop (#2312).
 * A rare case where delays in processing `Event`s generated by
   `ChannelMonitor`s could lead to loss of those events in case of an untimely
   crash. This could lead to the loss of an `Event::SpendableOutputs` (#2369).
 * Fixed a regression in 0.0.115 which caused `PendingHTLCsForwardable` events
   to be missed when processing phantom node receives. This caused such
   payments to be delayed until a further, unrelated HTLC came in (#2395).
 * Peers which are unresponsive to channel messages for several timer ticks are
   now disconnected to allow for on-reconnection state machine reset. This
   works around some issues in LND prior to 16.3 which can cause channels to
   hang and eventually force-close (#2293).
 * `ChannelManager::new` now requires the current time (either from a recent
   block header or the system clock), ensuring invoices created immediately
   after startup aren't already expired (#2372).
 * Resolved an issue where reading a `ProbabilisticScorer` on some platforms
   (e.g. iOS) can lead to a panic (#2322).
 * `ChannelConfig::max_dust_htlc_exposure` is now allowed to scale based on
   current fees, and the default has been updated to do so. This substantially
   reduces the chance of force-closure due to dust exposure. Note that existing
   channels will retain their current value and you may wish to update the
   value on your existing channels on upgrade (#2354).
 * `PeerManager::process_events` no longer blocks in any case. This fixes a bug
   where reentrancy from `PeerManager` into user code which eventually calls
   `process_events` could lead to a deadlock (#2280).
 * The persist timing of network graph and scoring in
   `lightning-background-processor` has been tweaked to provide more reliable
   persistence after updates to either (#2226).
 * The number of route hints added to BOLT 11 invoices by the
   `lightning-invoice::utils` builders has been reduced to three to ensure
   invoices can be represented in scan-able QR codes (#2044).
 * Fixed sending large onion messages, which would previously have resulted in
   an HMAC error on the second hop (#2277).
 * Fixed a memory leak that may occur when a `ChannelManager` or
   `ChannelMonitor` is `drop`ed (#2233).
 * A potential deadlock in calling `NetworkGraph::eq` was resolved (#2284).
 * Fixed an overflow which prevented disconnecting peers in some minor cases
   with more than 31 peers (#2245).
 * Gossip messages with an unknown chain hash are now ignored (#2230).
 * Rapid Gossip Sync processing now fails on an unknown chain hash (#2324).
 * `RouteHintHop::htlc_maximum_msat` is now enforced. Note that BOLT11 route
   hints do not have such a field so this code is generally unused (#2305).

## Security
0.0.116 fixes a denial-of-service vulnerability which is reachable from
untrusted input from channel counterparties if a 0-conf channel exists with
that counterparty.
 * A premature `announcement_signatures` message from a peer prior to a 0-conf
   channel's funding transaction receiving any confirmations would panic in any
   version since 0-conf channels were introduced (#2439).

In total, this release features 142 files changed, 21033 insertions, 11066
deletions in 327 commits from 21 authors, in alphabetical order:
 * Alec Chen
 * Andrei
 * Antoine Riard
 * Arik Sosman
 * Chad Upjohn
 * Daniel Granhão
 * Duncan Dean
 * Elias Rohrer
 * Fred Walker
 * Gleb Naumenko
 * Jeffrey Czyz
 * Martin Habovstiak
 * Matt Corallo
 * Tony Giorgio
 * Valentine Wallace
 * Vladimir Fomene
 * Willem Van Lint
 * Wilmer Paulino
 * benthecarman
 * ff
 * henghonglee


# 0.0.115 - Apr 24, 2023 - "Rebroadcast the Bugfixes"

## API Updates
 * The MSRV of the main LDK crates has been increased to 1.48 (#2107).
 * Attempting to claim an un-expired payment on a channel which has closed no
   longer fails. The expiry time of payments is exposed via
   `PaymentClaimable::claim_deadline` (#2148).
 * `payment_metadata` is now supported in `Invoice` deserialization, sending,
   and receiving (via a new `RecipientOnionFields` struct) (#2139, #2127).
 * `Event::PaymentFailed` now exposes a failure reason (#2142).
 * BOLT12 messages now support stateless generation and validation (#1989).
 * The `NetworkGraph` is now pruned of stale data after RGS processing (#2161).
 * Max inbound HTLCs in-flight can be changed in the handshake config (#2138).
 * `lightning-transaction-sync` feature `esplora-async-https` was added (#2085).
 * A `ChannelPending` event is now emitted after the initial handshake (#2098).
 * `PaymentForwarded::outbound_amount_forwarded_msat` was added (#2136).
 * `ChannelManager::list_channels_by_counterparty` was added (#2079).
 * `ChannelDetails::feerate_sat_per_1000_weight` was added (#2094).
 * `Invoice::fallback_addresses` was added to fetch `bitcoin` types (#2023).
 * The offer/refund description is now exposed in `Invoice{,Request}` (#2206).

## Backwards Compatibility
 * Payments sent with the legacy `*_with_route` methods on LDK 0.0.115+ will no
   longer be retryable via the LDK 0.0.114- `retry_payment` method (#2139).
 * `Event::PaymentPathFailed::retry` was removed and will always be `None` for
    payments initiated on 0.0.115 which fail on an earlier version (#2063).
 * `Route`s and `PaymentParameters` with blinded path information will not be
   readable on prior versions of LDK. Such objects are not currently constructed
   by LDK, but may be when processing BOLT12 data in a coming release (#2146).
 * Providing `ChannelMonitorUpdate`s generated by LDK 0.0.115 to a
   `ChannelMonitor` on 0.0.114 or before may panic (#2059). Note that this is
   in general unsupported, and included here only for completeness.

## Bug Fixes
 * Fixed a case where `process_events_async` may `poll` a `Future` which has
   already completed (#2081).
 * Fixed deserialization of `u16` arrays. This bug may have previously corrupted
   the historical buckets in a `ProbabilisticScorer`. Users relying on the
   historical buckets may wish to wipe their scorer on upgrade to remove corrupt
   data rather than waiting on it to decay (#2191).
 * The `process_events_async` task is now `Send` and can thus be polled on a
   multi-threaded runtime (#2199).
 * Fixed a missing macro export causing
   `impl_writeable_tlv_based_enum{,_upgradable}` calls to not compile (#2091).
 * Fixed compilation of `lightning-invoice` with both `no-std` and serde (#2187)
 * Fix an issue where the `background-processor` would not wake when a
   `ChannelMonitorUpdate` completed asynchronously, causing delays (#2090).
 * Fix an issue where `process_events_async` would exit immediately (#2145).
 * `Router` calls from the `ChannelManager` now call `find_route_with_id` rather
   than `find_route`, as was intended and described in the API (#2092).
 * Ensure `process_events_async` always exits if any sleep future returns true,
   not just if all sleep futures repeatedly return true (#2145).
 * `channel_update` messages no longer set the disable bit unless the peer has
   been disconnected for some time. This should resolve cases where channels are
   disabled for extended periods of time (#2198).
 * We no longer remove CLN nodes from the network graph for violating the BOLT
   spec in some cases after failing to pay through them (#2220).
 * Fixed a debug assertion which may panic under heavy load (#2172).
 * `CounterpartyForceClosed::peer_msg` is now wrapped in UntrustedString (#2114)
 * Fixed a potential deadlock in `funding_transaction_generated` (#2158).

## Security
 * Transaction re-broadcasting is now substantially more aggressive, including a
   new regular rebroadcast feature called on a timer from the
   `background-processor` or from `ChainMonitor::rebroadcast_pending_claims`.
   This should substantially increase transaction confirmation reliability
   without relying on downstream `TransactionBroadcaster` implementations for
   rebroadcasting (#2203, #2205, #2208).
 * Implemented the changes from BOLT PRs #1031, #1032, and #1040 which resolve a
   privacy vulnerability which allows an intermediate node on the path to
   discover the final destination for a payment (#2062).

In total, this release features 110 files changed, 11928 insertions, 6368
deletions in 215 commits from 21 authors, in alphabetical order:
 * Advait
 * Alan Cohen
 * Alec Chen
 * Allan Douglas R. de Oliveira
 * Arik Sosman
 * Elias Rohrer
 * Evan Feenstra
 * Jeffrey Czyz
 * John Cantrell
 * Lucas Soriano del Pino
 * Marc Tyndel
 * Matt Corallo
 * Paul Miller
 * Steven
 * Steven Williamson
 * Steven Zhao
 * Tony Giorgio
 * Valentine Wallace
 * Wilmer Paulino
 * benthecarman
 * munjesi


# 0.0.114 - Mar 3, 2023 - "Faster Async BOLT12 Retries"

## API Updates
 * `InvoicePayer` has been removed and its features moved directly into
   `ChannelManager`. As such it now requires a simplified `Router` and supports
   `send_payment_with_retry` (and friends). `ChannelManager::retry_payment` was
   removed in favor of the automated retries. Invoice payment utilities in
   `lightning-invoice` now call the new code (#1812, #1916, #1929, #2007, etc).
 * `Sign`/`BaseSign` has been renamed `ChannelSigner`, with `EcdsaChannelSigner`
   split out in anticipation of future schnorr/taproot support (#1967).
 * The catch-all `KeysInterface` was split into `EntropySource`, `NodeSigner`,
   and `SignerProvider`. `KeysManager` implements all three (#1910, #1930).
 * `KeysInterface::get_node_secret` is now `KeysManager::get_node_secret_key`
   and is no longer required for external signers (#1951, #2070).
 * A `lightning-transaction-sync` crate has been added which implements keeping
   LDK in sync with the chain via an esplora server (#1870). Note that it can
   only be used on nodes that *never* ran a previous version of LDK.
 * `Score` is updated in `BackgroundProcessor` instead of via `Router` (#1996).
 * `ChainAccess::get_utxo` (now `UtxoAccess`) can now be resolved async (#1980).
 * BOLT12 `Offer`, `InvoiceRequest`, `Invoice` and `Refund` structs as well as
   associated builders have been added. Such invoices cannot yet be paid due to
   missing support for blinded path payments (#1927, #1908, #1926).
 * A `lightning-custom-message` crate has been added to make combining multiple
   custom messages into one enum/handler easier (#1832).
 * `Event::PaymentPathFailed` is now generated for failure to send an HTLC
   over the first hop on our local channel (#2014, #2043).
 * `lightning-net-tokio` no longer requires an `Arc` on `PeerManager` (#1968).
 * `ChannelManager::list_recent_payments` was added (#1873).
 * `lightning-background-processor` `std` is now optional in async mode (#1962).
 * `create_phantom_invoice` can now be used in `no-std` (#1985).
 * The required final CLTV delta on inbound payments is now configurable (#1878)
 * bitcoind RPC error code and message are now surfaced in `block-sync` (#2057).
 * Get `historical_estimated_channel_liquidity_probabilities` was added (#1961).
 * `ChannelManager::fail_htlc_backwards_with_reason` was added (#1948).
 * Macros which implement serialization using TLVs or straight writing of struct
   fields are now public (#1823, #1976, #1977).

## Backwards Compatibility
 * Any inbound payments with a custom final CLTV delta will be rejected by LDK
   if you downgrade prior to receipt (#1878).
 * `Event::PaymentPathFailed::network_update` will always be `None` if an
   0.0.114-generated event is read by a prior version of LDK (#2043).
 * `Event::PaymentPathFailed::all_paths_failed` will always be false if an
   0.0.114-generated event is read by a prior version of LDK. Users who rely on
   it to determine payment retries should migrate to `Event::PaymentFailed`, in
   a separate release prior to upgrading to LDK 0.0.114 if downgrading is
   supported (#2043).

## Performance Improvements
 * Channel data is now stored per-peer and channel updates across multiple
   peers can be operated on simultaneously (#1507).
 * Routefinding is roughly 1.5x faster (#1799).
 * Deserializing a `NetworkGraph` is roughly 6x faster (#2016).
 * Memory usage for a `NetworkGraph` has been reduced substantially (#2040).
 * `KeysInterface::get_secure_random_bytes` is roughly 200x faster (#1974).

## Bug Fixes
 * Fixed a bug where a delay in processing a `PaymentSent` event longer than the
   time taken to persist a `ChannelMonitor` update, when occurring immediately
   prior to a crash, may result in the `PaymentSent` event being lost (#2048).
 * Fixed spurious rejections of rapid gossip sync data when the graph has been
   updated by other means between gossip syncs (#2046).
 * Fixed a panic in `KeysManager` when the high bit of `starting_time_nanos`
   is set (#1935).
 * Resolved an issue where the `ChannelManager::get_persistable_update_future`
   future would fail to wake until a second notification occurs (#2064).
 * Resolved a memory leak when using `ChannelManager::send_probe` (#2037).
 * Fixed a deadlock on some platforms at least when using async `ChannelMonitor`
   updating (#2006).
 * Removed debug-only assertions which were reachable in threaded code (#1964).
 * In some cases when payment sending fails on our local channel retries no
   longer take the same path and thus never succeed (#2014).
 * Retries for spontaneous payments have been fixed (#2002).
 * Return an `Err` if `lightning-persister` fails to read the directory listing
   rather than panicing (#1943).
 * `peer_disconnected` will now never be called without `peer_connected` (#2035)

## Security
0.0.114 fixes several denial-of-service vulnerabilities which are reachable from
untrusted input from channel counterparties or in deployments accepting inbound
connections or channels. It also fixes a denial-of-service vulnerability in rare
cases in the route finding logic.
 * The number of pending un-funded channels as well as peers without funded
   channels is now limited to avoid denial of service (#1988).
 * A second `channel_ready` message received immediately after the first could
   lead to a spurious panic (#2071). This issue was introduced with 0conf
   support in LDK 0.0.107.
 * A division-by-zero issue was fixed in the `ProbabilisticScorer` if the amount
   being sent (including previous-hop fees) is equal to a channel's capacity
   while walking the graph (#2072). The division-by-zero was introduced with
   historical data tracking in LDK 0.0.112.

In total, this release features 130 files changed, 21457 insertions, 10113
deletions in 343 commits from 18 authors, in alphabetical order:
 * Alec Chen
 * Allan Douglas R. de Oliveira
 * Andrei
 * Arik Sosman
 * Daniel Granhão
 * Duncan Dean
 * Elias Rohrer
 * Jeffrey Czyz
 * John Cantrell
 * Kurtsley
 * Matt Corallo
 * Max Fang
 * Omer Yacine
 * Valentine Wallace
 * Viktor Tigerström
 * Wilmer Paulino
 * benthecarman
 * jurvis


# 0.0.113 - Dec 16, 2022 - "Big Movement Intercepted"

## API Updates
 * `ChannelManager::send_payment` now takes an explicit `PaymentId` which is a
   loose idempotency token. See `send_payment` docs for more (#1761, #1826).
 * HTLCs bound for SCIDs from `ChannelManager::get_intercept_scid` are now
   intercepted and can be forwarded manually over any channel (#1835, #1893).
 * `Confirm::get_relevant_txids` now returns a `BlockHash`, expanding the set
   of cases where `transaction_unconfirmed` must be called, see docs (#1796).
 * Pending outbound payments are no longer automatically timed-out a few blocks
   after failure. Thus, in order to avoid leaking memory, you MUST call
   `ChannelManager::abandon_payment` when you no longer wish to retry (#1761).
 * `ChannelManager::abandon_payment` docs were updated to note that the payment
   may return to pending after a restart if no persistence occurs (#1907).
 * `Event::PaymentReceived` has been renamed `Event::PaymentClaimable` (#1891).
 * `Event` handling is now optionally async for Rust users (#1787).
 * `user_channel_id` is now a `u128` and random for inbound channels (#1790).
 * A new `ChannelReady` event is generated whenever a channel becomes ready to
   be used, i.e., after both sides sent the `channel_ready` message (#1743).
 * `NetworkGraph` now prunes channels where either node is offline for 2 weeks
   and refuses to accept re-announcements of pruned channels (#1735).
 * Onion messages are now read in `CustomOnionMessageHandler` rather than via
   `MaybeReadableArgs` (#1809).
 * Added a new util to generate an invoice with a custom hash (#1894) -
`create_invoice_from_channelmanager_and_duration_since_epoch_with_payment_hash`
 * `Sign`ers are now by default re-derived using `KeysInterface`'s new
   `derive_channel_signer` rather than `read_chan_signer` (#1867).
 * `Confirm::transactions_confirmed` is now idempotent (#1861).
 * `ChannelManager::compute_inflight_htlcs` has been added to fetch in-flight
   HTLCs for scoring. Note that `InvoicePayer` does this for you (#1830).
 * Added `PaymentClaimable::via_channel_id` (#1856).
 * Added the `node_id` (phantom or regular) to payment events (#1766).
 * Added the funding transaction `confirmations` to `ChannelDetails` (#1856).
 * `BlindedRoute` has been renamed `BlindedPath` (#1918).
 * Support for the BOLT 4 "legacy" onion format has been removed, in line with
   its removal in the spec and vanishingly rare use (#1413).
 * `ChainMonitor::list_pending_monitor_updates` was added (#1834).
 * Signing for non-zero-fee anchor commitments is supported again (#1828).
 * Several helpers for transaction matching and generation are now pub (#1839).

## Bug Fixes
 * Fixed a rare race where a crash may result in a pending HTLC not being
   failed backwards, leading to a force-closure by our counterparty (#1857).
 * Avoid incorrectly assigning a lower-bound on channel liquidity when routing
   fails due to a closed channel earlier in the path (#1817).
 * If a counterparty increases the channel fee, but not enough per our own fee
   estimator, we no longer force-close the channel (#1852).
 * Several bugs in the `lightning-background-processor` `future` feature were
   fixed, including requirements doc corrections (#1843, #1845, #1851).
 * Some failure messages sent back when failing an HTLC were corrected (#1895).
 * `rapid-gossip-sync` no longer errors if an update is applied duplicatively
   or in rare cases when the graph is updated from payment failures (#1833).
 * Sending onion messages to a blinded path in which we're the introduction
   node no longer fails (#1791).

## Backwards Compatibility
 * No `ChannelReady` events will be generated for previously existing channels,
   including those which become ready after upgrading to 0.0.113 (#1743).
 * Once `UserConfig::accept_intercept_htlcs` is set, downgrades to LDK versions
   prior to 0.0.113 are not supported (#1835).
 * Existing payments may see a `PaymentClaimable::user_channel_id` of 0 (#1856)
 * When downgrading to a version of LDK prior to 0.0.113 when there are
   resolved payments waiting for a small timeout, the payments may not be
   removed, preventing payments with the same `PaymentId` (#1761).

In total, this release features 76 files changed, 11639 insertions, 6067
deletions in 210 commits from 18 authors, in alphabetical order:
 * Antoine Riard
 * Arik Sosman
 * Devrandom
 * Duncan Dean
 * Elias Rohrer
 * Gleb Naumenko
 * Jeffrey Czyz
 * John Cantrell
 * Matt Corallo
 * Tee8z
 * Tobin C. Harding
 * Tristan F
 * Valentine Wallace
 * Viktor Tigerström
 * Wilmer Paulino
 * benthecarman
 * jurvis
 * ssbright


# 0.0.112 - Oct 25, 2022 - "History Matters"

## API Updates
 * `Result<(), ChannelMonitorUpdateErr>` return values have been replaced with
   a `ChannelMonitorUpdateStatus` trinary enum. This better denotes that
   `ChannelMonitorUpdateStatus::InProgress` is not an error, but asynchronous
   persistence of a monitor update. Note that asynchronous persistence still
   has some edge cases and is not yet recommended for production (#1106).
 * `ChannelMonitor` persistence failure no longer automatically broadcasts the
   latest commitment transaction. See the
   `ChannelMonitorUpdateStatus::PermanentFailure` docs for more info (#1106).
 * `*Features::known` has been replaced with individual
   `*MessageHandler::provided_*_features` methods (#1707).
 * `OnionMessenger` now takes a `CustomOnionMessageHandler` implementation,
   allowing you to send and receive custom onion messages (#1748).
 * `ProbabilisticScorer` now tracks the historical distribution of liquidity
   estimates for channels. See new `historical_*` parameters in
   `ProbabilisticScoringParameters` for more details (#1625).
 * `lightning-block-sync`'s `BlockSource` trait now supports BIP 157/158
   filtering clients by returning only header data for some blocks (#1706).
 * `lightning-invoice`'s `Router` trait now accepts an `InFlightHtlcs` to
   ensure we do not over-use a remote channel's funds during routing (#1694).
   Note that this was previously backported to 0.0.111 for bindings users.
 * `NetworkGraph::remove_stale_channels` has been renamed
   `NetworkGraph::remove_stale_channels_and_tracking` as `NetworkGraph` now
   refuses to re-add nodes and channels that were recently removed (#1649).
 * The `lightning-rapid-gossip-sync` crate now supports `no-std` (#1708).
 * The default `ProbabilisticScoringParameters::liquidity_offset_half_life` has
   been increased to six hours from one (#1754).
 * All commitment transaction building logic for anchor outputs now assumes the
   no-HTLC-tx-fee variant (#1685).
 * A number of missing `Eq` implementations were added (#1763).

## Bug Fixes
 * `lightning-background-processor` now builds without error with the `futures`
   feature (#1744).
 * `ChannelManager::get_persistable_update_future`'s returned `Future` has been
   corrected to not fail to be awoken in some cases (#1758).
 * Asynchronously performing the initial `ChannelMonitor` persistence is now
   safe (#1678).
 * Redundantly applying rapid gossip sync updates no longer `Err`s (#1764).
 * Nodes which inform us via payment failures that they should no longer be
   used are now removed from the network graph. Some LND nodes spuriously
   generate this error and may remove themselves from our graph (#1649).

In total, this release features 134 files changed, 6598 insertions, 4370
deletions in 109 commits from 13 authors, in alphabetical order:
 * Duncan Dean
 * Elias Rohrer
 * Gabriel Comte
 * Gursharan Singh
 * Jeffrey Czyz
 * Jurvis Tan
 * Matt Corallo
 * Max Fang
 * Paul Miller
 * Valentine Wallace
 * Viktor Tigerström
 * Wilmer Paulino
 * acid-bit

# 0.0.111 - Sep 12, 2022 - "Saturated with Messages"

## API Updates
 * Support for relaying onion messages has been added via a new
   `OnionMessenger` struct when passed as the `OnionMessageHandler` to a
   `PeerManager`. Pre-encoded onion messages can also be sent and received
   (#1503, #1650, #1652, #1688).
 * Rate-limiting of outbound gossip syncs has been rewritten to utilize less
   buffering inside LDK. The new rate-limiting is also used for onion messages
   to avoid delaying other messages (#1604. #1660, #1683).
 * Rather than spawning a full OS thread, `lightning-background-processor` has
   a new `process_events_async` method which takes the place of a
   `BackgroundProcessor` for those using Rust's async (#1657).
 * `ChannelManager::get_persistable_update_future` has been added to block on
   a ChannelManager needing re-persistence in a Rust async environment (#1657).
 * The `Filter::register_output` return value has been removed, as it was
   very difficult to correctly implement (i.e., without blocking). Users
   previously using it should instead pass dependent transactions in via
   additional `chain::Confirm::transactions_confirmed` calls (#1663).
 * `ChannelHandshakeConfig::their_channel_reserve_proportional_millionths` has
   been added to allow configuring counterparty reserve values (#1619).
 * `KeysInterface::ecdh` has been added as an ECDH oracle (#1503, #1658).
 * The `rust-bitcoin` dependency has been updated 0.29 (#1658).
 * The `bitcoin_hashes` dependency has been updated 0.11 (#1677).
 * `ChannelManager::broadcast_node_announcement` has been moved to
   `PeerManager` (#1699).
 * `channel_` and `node_announcement`s are now rebroadcast automatically to all
   new peers which connect (#1699).
 * `{Init,Node}Features` sent to peers/broadcasted are now fetched via the
   various `*MessageHandler` traits, rather than hard-coded (#1701, #1688).
 * `Event::PaymentPathFailed::rejected_by_dest` has been renamed
   `payment_failed_permanently` (#1702).
 * `Invoice` now derives the std `Hash` trait (#1575).
 * `{Signed,}RawInvoice::hash` have been renamed `signable_hash` (#1714).
 * `chain::AccessError` now derives the std `Debug` trait (#1709).
 * `ReadOnlyNetworkGraph::list_{channels,nodes}` have been added largely for
   users of downstream bindings (#1651).
 * `ChannelMonitor::get_counterparty_node_id` is now available (#1635).

## Bug Fixes
 * The script compared with that returned from `chain::Access` was incorrect
   ~half of the time, causing spurious gossip rejection (#1666).
 * Pending in-flight HTLCs are now considered when calculating new routes,
   ensuring, e.g. MPP retries do not take known-saturated paths (#1643).
 * Counterparty-revoked outputs are now included in `get_claimable_balance`
   output via a new `Balance::CounterpartyRevokedOutputClaimable` (#1495).
 * Inbound HTLCs for which we do not (yet) have a preimage are now included in
   `get_claimable_balance` via a `Balance::MaybePreimageClaimableHTLC` (#1673).
 * Probes that fail prior to being sent over their first hop are correctly
   failed with a `Event::ProbeFailed` rather than a `PaymentPathFailed` (#1704).
 * Pending `Event::HTLCHandlingFailed`s are no longer lost on restart (#1700).
 * HTLCs that fail prior to being sent over their first hop are now marked as
   retryable via `!PaymentPathFailed::payment_failed_permanently` (#1702).
 * Dust HTLCs are now considered failed in the payment tracking logic after the
   commitment transaction confirms, allowing retry on restart (#1691).
 * On machines with buggy "monotonic" clocks, LDK will no longer panic if time
   goes backwards (#1692).

## Backwards Compatibility
 * The new `current_time` argument to `PeerManager` constructors must be set to
   a UNIX timestamp for upgraded nodes; new nodes may use a counter (#1699).
 * `Balance::CounterpartyRevokedOutputClaimable` will never be generated for
   channels that were observed to go on-chain with LDK versions prior to
   0.0.111 (#1495).
 * `ChannelMonitor::get_counterparty_node_id` will return `None` for all
   channels opened on a version of LDK prior to 0.0.110 (#1635).
 * Setting `their_channel_reserve_proportional_millionths` to any value other
   than the default will cause LDK versions prior to 0.0.104 to be unable to
   read the serialized `ChannelManager` (#1619).

## Security
0.0.111 fixes a denial-of-service vulnerability which is reachable from
untrusted input in deployments accepting 0conf channels, or via a race-condition
in deployments creating outbound 0conf channels.

 * LDK versions prior to 0.0.111 may spuriously panic when receiving a block if
   they are awaiting the construction of a funding transaction for a 0-conf
   channel (#1711). 0-conf support was added in LDK version 0.0.107.

In total, this release features 84 files changed, 6306 insertions, 1960
deletions in 121 commits from 11 authors, in alphabetical order:
 * Arik Sosman
 * Devrandom
 * Duncan Dean
 * Elias Rohrer
 * Gursharan Singh
 * Matt Corallo
 * NicolaLS
 * Valentine Wallace
 * Viktor Tigerström
 * jurvis
 * ok300


# 0.0.110 - 2022-07-26 - "Routing, With a Vengeance"

## API Updates
 * `ChannelManager::send_probe` and `Score::probe_{failed,successful}` have
   been added to make probing more explicit, as well as new
   `Event::Probe{Failed,Successful}` events (#1567).
 * `ProbabilisticScoringParameters::banned_nodes` has been renamed
   `manual_node_penalties` and changed to take msat penalties (#1592).
 * Per-payment tracking of failed paths was added to enable configuration of
   `ProbabilisticScoringParameters::considered_impossible_penalty_msat` (#1600)
 * `ProbabilisticScoringParameters::base_penalty_amount_multiplier_msat` was
   added to allow a penalty that is only amount-dependent (#1617).
 * `ProbabilisticScoringParameters::amount_penalty_multiplier_msat` was renamed
   `liquidity_penalty_amount_multiplier_msat` (#1617).
 * A new `Event::HTLCHandlingFailed` has been added which provides visibility
   into failures to forward/claim accepted HTLCs (#1403).
 * Support has been added for DNS hostnames in the `NetAddress` type, see
   [BOLT PR #911](https://github.com/lightning/bolts/pull/911) (#1553).
 * `GossipSync` now has `rapid`, `p2p`, and `none` constructors (#1618).
 * `lightning-net-tokio` no longer requires types to be in `Arc`s (#1623).
 * The `htlc_maximum_msat` field is now required in `ChannelUpdate` gossip
   messages. In tests this rejects < 1% of channels (#1519).
 * `ReadOnlyNetworkGraph::{channel,node}` have been added to query for
   individual channel/node data, primarily for bindings users (#1543).
 * `FeeEstimator` implementations are now wrapped internally to ensure values
   below 253 sats/kW are never used (#1552).
 * Route selection no longer attempts to randomize path selection. This is
   unlikely to lead to a material change in the paths selected (#1610).

## Bug Fixes
 * Fixed a panic when deserializing `ChannelDetails` objects (#1588).
 * When routing, channels are no longer fully saturated before MPP splits are
   generated, instead a configuration knob was added as
   `PaymentParameters::max_channel_saturation_power_of_half` (#1605).
 * Fixed a panic which occurred in `ProbabilisticScorer` when wallclock time
   goes backwards across a restart (#1603).

## Serialization Compatibility
 * All new fields are ignored by prior versions of LDK. All new fields are not
   present when reading objects serialized by prior versions of LDK.
 * Channel information written in the `NetworkGraph` which is missing
   `htlc_maximum_msat` may be dropped on deserialization (#1519).
 * Similarly, node information written in the `NetworkGraph` which contains an
   invalid hostname may be dropped on deserialization (#1519).

In total, this release features 79 files changed, 2935 insertions, 1363
deletions in 52 commits from 9 authors, in alphabetical order:
 * Duncan Dean
 * Elias Rohrer
 * Jeffrey Czyz
 * Matt Corallo
 * Max Fang
 * Viktor Tigerström
 * Willem Van Lint
 * Wilmer Paulino
 * jurvis

# 0.0.109 - 2022-07-01 - "The Kitchen Sink"

## API Updates
 * `ChannelManager::update_channel_config` has been added to allow the fields
   in `ChannelConfig` to be changed in a given channel after open (#1527).
 * If we reconnect to a peer which proves we have a stale channel state, rather
   than force-closing we will instead panic to provide an opportunity to switch
   to the latest state and continue operating without channel loss (#1564).
 * A `NodeAlias` struct has been added which handles string sanitization for
   node aliases via the `Display` trait (#1544).
 * `ProbabilisticScoringParameters` now has a `banned_nodes` set which we will
    never route through during path finding (#1550).
 * `ProbabilisticScoringParameters` now offers an `anti_probing_penalty_msat`
   option to prefer channels which afford better privacy when routing (#1555).
 * `ProbabilisticScorer` now provides access to its estimated liquidity range
   for a given channel via `estimated_channel_liquidity_range` (#1549).
 * `ChannelManager::force_close_channel` has been renamed
   `force_close_broadcasting_latest_txn` and
   `force_close_without_broadcasting_txn` has been added (#1564).
 * Options which cannot be changed at runtime have been moved from
   `ChannelConfig` to `ChannelHandshakeConfig` (#1529).
 * `find_route` takes `&NetworkGraph` instead of `ReadOnlyNetworkGraph (#1583).
 * `ChannelDetails` now contains a copy of the current `ChannelConfig` (#1527).
 * The `lightning-invoice` crate now optionally depends on `serde`, with
   `Invoice` implementing `serde::{Deserialize,Serialize}` if enabled (#1548).
 * Several fields in `UserConfig` have been renamed for clarity (#1540).

## Bug Fixes
 * `find_route` no longer selects routes with more than
   `PaymentParameters::max_mpp_path_count` paths, and
   `ChannelManager::send_payment` no longer refuses to send along routes with
   more than ten paths (#1526).
 * Fixed two cases where HTLCs pending at the time a counterparty broadcasts a
   revoked commitment transaction are considered resolved prior to their actual
   resolution on-chain, possibly passing the update to another channel (#1486).
 * HTLCs which are relayed through LDK may now have a total expiry time two
   weeks in the future, up from one, reducing forwarding failures (#1532).

## Serialization Compatibility
 * All new fields are ignored by prior versions of LDK. All new fields are not
   present when reading objects serialized by prior versions of LDK.
 * `ChannelConfig`'s serialization format has changed and is not compatible
   with any previous version of LDK. Attempts to read values written by a
   previous version of LDK will fail and attempts to read newly written objects
   using a previous version of LDK will fail. It is not expected that users are
   serializing `ChannelConfig` using the LDK serialization API, however, if a
   backward compatibility wrapper is required, please open an issue.

## Security
0.0.109 fixes a denial-of-service vulnerability which is reachable from
untrusted input in some application deployments.

 * Third parties which are allowed to open channels with an LDK-based node may
   fund a channel with a bogus and maliciously-crafted transaction which, when
   spent, can cause a panic in the channel's corresponding `ChannelMonitor`.
   Such a channel is never usable as it cannot be funded with a funding
   transaction which matches the required output script, allowing the
   `ChannelMonitor` for such channels to be safely purged as a workaround on
   previous versions of LDK. Thanks to Eugene Siegel for reporting this issue.

In total, this release features 32 files changed, 1948 insertions, 532
deletions in 33 commits from 9 authors, in alphabetical order:
 * Antoine Riard
 * Daniel Granhão
 * Elias Rohrer
 * Jeffrey Czyz
 * Matt Corallo
 * Matt Faltyn
 * NicolaLS
 * Valentine Wallace
 * Wilmer Paulino


# 0.0.108 - 2022-06-10 - "You Wanted It To Build?! Why Didn't You Say So?"

## Bug Fixes
 * Fixed `lightning-background-processor` build in release mode.

In total, this release features 9 files changed, 120 insertions, 74
deletions in 5 commits from 4 authors, in alphabetical order:
 * Elias Rohrer
 * Matt Corallo
 * Max Fang
 * Viktor Tigerström

# 0.0.107 - 2022-06-08 - "BlueWallet's Wishlist"

## API Updates
 * Channels larger than 16777215 sats (Wumbo!) are now supported and can be
   enabled for inbound channels using
   `ChannelHandshakeLimits::max_funding_satoshis` (#1425).
 * Support for feature `option_zeroconf`, allowing immediate forwarding of
   payments after channel opening. This is configured for outbound channels
   using `ChannelHandshakeLimits::trust_own_funding_0conf` whereas
   `ChannelManager::accept_inbound_channel_from_trusted_peer_0conf` has to be
   used for accepting inbound channels (#1401, #1505).
 * `ChannelManager::claim_funds` no longer returns a `bool` to indicate success.
   Instead, an `Event::PaymentClaimed` is generated if the claim was successful.
   Likewise, `ChannelManager::fail_htlc_backwards` no longer has a return value
   (#1434).
 * `lightning-rapid-gossip-sync` is a new crate for syncing gossip data from a
   server, primarily aimed at mobile devices (#1155).
 * `RapidGossipSync` can be passed to `BackgroundProcessor` in order to persist
   the `NetworkGraph` and handle `NetworkUpdate`s during event handling (#1433,
   #1517).
 * `NetGraphMsgHandler` has been renamed to `P2PGossipSync`, the `network_graph`
    module has been renamed to `gossip`, and `NetworkUpdate::ChannelClosed` has
   been renamed `NetworkUpdate::ChannelFailure` (#1159).
 * Added a `filtered_block_connected` method to `chain::Listen` and a default
   implementation of `block_connected` for those fetching filtered instead of
   full blocks (#1453).
 * The `lightning-block-sync` crate's `BlockSource` trait methods now take
   `&self` instead of `&mut self` (#1307).
 * `inbound_payment` module is now public to allow for creating invoices without
   a `ChannelManager` (#1384).
 * `lightning-block-sync`'s `init` and `poll` modules support `&dyn BlockSource`
   which can be determined at runtime (#1423).
 * `lightning-invoice` crate's `utils` now accept an expiration time (#1422,
   #1474).
 * `Event::PaymentForwarded` includes `prev_channel_id` and `next_channel_id`
   (#1419, #1475).
 * `chain::Watch::release_pending_monitor_events`' return type now associates
   `MonitorEvent`s with funding `OutPoints` (#1475).
 * `lightning-background-processor` crate's `Persister` trait has been moved to
   `lightning` crate's `util::persist` module, which now has a general
   `KVStorePersister` trait. Blanket implementations of `Persister` and
   `chainmonitor::Persist` are given for types implementing `KVStorePersister`.
   ` lightning-persister`'s `FilesystemPersister` implements `KVStorePersister`
   (#1417).
 * `ChannelDetails` and `ChannelCounterparty` include fields for HTLC minimum
   and maximum values (#1378).
 * Added a `max_inbound_htlc_value_in_flight_percent_of_channel` field to
   `ChannelHandshakeConfig`, capping the total value of outstanding inbound
   HTLCs for a channel (#1444).
 * `ProbabilisticScorer` is parameterized by a `Logger`, which it uses to log
   channel liquidity updates or lack thereof (#1405).
 * `ChannelDetails` has an `outbound_htlc_limit_msat` field, which should be
   used in routing instead of `outbound_capacity_msat` (#1435).
 * `ProbabilisticScorer`'s channel liquidities can be logged via
   `debug_log_liquidity_stats` (#1460).
 * `BackgroundProcessor` now takes an optional `WriteableScore` which it will
   persist using the `Persister` trait's new `persist_scorer` method (#1416).
 * Upgraded to `bitcoin` crate version 0.28.1 (#1389).
 * `ShutdownScript::new_witness_program` now takes a `WitnessVersion` instead of
   a `NonZeroU8` (#1389).
 * Channels will no longer be automatically force closed when the counterparty
   is disconnected due to incompatibility (#1429).
 * `ChannelManager` methods for funding, accepting, and closing channels now
   take a `counterparty_node_id` parameter, which has also been added as a field
   to `Event::FundingGenerationReady` (#1479, #1485).
 * `InvoicePayer::new` now takes a `Retry` enum (replacing the `RetryAttempts`
   struct), which supports both attempt- and timeout-based retrying (#1418).
 * `Score::channel_penalty_msat` takes a `ChannelUsage` struct, which contains
   the capacity as an `EffectiveCapacity` enum and any potential in-flight HTLC
   value, rather than a single `u64`. Used by `ProbabilisticScorer` for more
   accurate penalties (#1456).
 * `build_route_from_hops` is a new function useful for constructing a `Route`
   given a specific list of public keys (#1491).
 * `FundingLocked` message has been renamed `ChannelReady`, and related
   identifiers have been renamed accordingly (#1506).
 * `core2::io` or `std::io` (depending on feature flags `no-std` or `std`) is
   exported as a `lightning::io` module (#1504).
 * The deprecated `Scorer` has been removed in favor or `ProbabilisticScorer`
   (#1512).

## Performance Improvements
 * `lightning-persister` crate's `FilesystemPersister` is faster by 15x (#1404).
 * Log gossip query messages at `GOSSIP` instead of `TRACE` to avoid
   overwhelming default logging (#1421).
 * `PeerManager` supports processing messages from different peers in parallel,
   and this is taken advantage of in gossip processing (#1023).
 * Greatly reduced per-channel and per-node memory usage due to upgrade of
   `secp256k1` crate to 0.22.1 and `bitcoin` crate to 0.28.1
 * Reduced per-peer memory usage in `PeerManager` (#1472).

## Spec Compliance
 * `find_route` now assumes variable-length onions by default for nodes where
   support for the feature is unknown (#1414).
 * A `warn` message is now sent when receiving a `channel_reestablish` with an
   old commitment transaction number rather than immediately force-closing the
   channel (#1430).
 * When a `channel_update` message is included in an onion error's `failuremsg`,
   its message type is now encoded. Reading such messages is also supported
   (#1465).

## Bug Fixes
 * Fixed a bug where crashing while persisting a `ChannelMonitorUpdate` for a
   part of a multi-path payment could cause loss of funds due to a partial
   payment claim on restart (#1434).
 * `BackgroundProcessor` has been fixed to improve serialization reliability on
   slow systems which can avoid force-closes (#1436).
 * `gossip_timestamp_filter` filters are now honored when sending gossip to
   peers (#1452).
 * During a reorg, only force-close a channel if its funding transaction is
   unconfirmed rather than as it loses confirmations (#1461).
 * Fixed a rare panic in `lightning-net-tokio` when fetching a peer's socket
   address after the connection has been closed caused by a race condition
   (#1449).
 * `find_route` will no longer return routes that would cause onion construction
   to fail in some cases (#1476).
 * `ProbabilisticScorer` uses more precision when approximating `log10` (#1406).

## Serialization Compatibility
 * All above new events/fields are ignored by prior clients. All above new
   events/fields are not present when reading objects serialized by prior
   versions of the library.
 * `ChannelManager` serialization is no longer compatible with versions prior to
   0.0.99 (#1401).
 * Channels with `option_zeroconf` feature enabled (not required for 0-conf
   channel use) will be unreadable by versions prior to 0.0.107 (#1401, #1505).

In total, this release features 96 files changed, 9304 insertions, 4503
deletions in 153 commits from 18 authors, in alphabetical order:
 * Arik Sosman
 * Devrandom
 * Duncan Dean
 * Elias Rohrer
 * Jeffrey Czyz
 * John Cantrell
 * John Corser
 * Jurvis Tan
 * Justin Moon
 * KaFai Choi
 * Matt Faltyn
 * Matt Corallo
 * Valentine Wallace
 * Viktor Tigerström
 * Vincenzo Palazzo
 * atalw
 * dependabot[bot]
 * shamardy


# 0.0.106 - 2022-04-03

## API Updates
 * Minimum supported rust version (MSRV) is now 1.41.1 (#1310).
 * Lightning feature `option_scid_alias` is now supported and may be negotiated
   when opening a channel with a peer. It can be configured via
   `ChannelHandshakeConfig::negotiate_scid_privacy` and is off by default but
   will be on by default in the future (#1351).
 * `OpenChannelRequest` now has a `channel_type` field indicating the features
   the channel will operate with and should be used to filter channels with
   undesirable features (#1351). See the Serialization Compatibility section.
 * `ChannelManager` supports sending and receiving short channel id aliases in
   the `funding_locked` message. These are used when forwarding payments and
   constructing invoice route hints for improved privacy. `ChannelDetails` has a
   `inbound_scid_alias` field and a `get_inbound_payment_scid` method to support
   the latter (#1311).
 * `DefaultRouter` and `find_route` take an additional random seed to improve
   privacy by adding a random CLTV expiry offset to each path's final hop. This
   helps obscure the intended recipient from adversarial intermediate hops
   (#1286). The seed is  also used to randomize candidate paths during route
   selection (#1359).
 * The `lightning-block-sync` crate's `init::synchronize_listeners` method
   interface has been relaxed to support multithreaded environments (#1349).
 * `ChannelManager::create_inbound_payment_for_hash`'s documentation has been
   corrected to remove the one-year restriction on `invoice_expiry_delta_secs`,
   which is only applicable to the deprecated `create_inbound_payment_legacy`
   and `create_inbound_payment_for_hash_legacy` methods (#1341).
 * `Features` mutator methods now take `self` by reference instead of by value
   (#1331).
 * The CLTV of the last hop in a path is now included when comparing against
   `RouteParameters::max_total_cltv_expiry_delta` (#1358).
 * Invoice creation functions in `lightning-invoice` crate's `utils` module
   include versions that accept a description hash instead of only a description
   (#1361).
 * `RoutingMessageHandler::sync_routing_table` has been renamed `peer_connected`
   (#1368).
 * `MessageSendEvent::SendGossipTimestampFilter` has been added to indicate that
   a `gossip_timestamp_filter` should be sent (#1368).
 * `PeerManager` takes an optional `NetAddress` in `new_outbound_connection` and
   `new_inbound_connection`, which is used to report back the remote address to
   the connecting peer in the `init` message (#1326).
 * `ChannelManager::accept_inbound_channel` now takes a `user_channel_id`, which
   is used in a similar manner as in outbound channels. (#1381).
 * `BackgroundProcessor` now persists `NetworkGraph` on a timer and upon
   shutdown as part of a new `Persister` trait, which also includes
   `ChannelManager` persistence (#1376).
 * `ProbabilisticScoringParameters` now has a `base_penalty_msat` option, which
   default to 500 msats. It is applied at each hop to help avoid longer paths
   (#1375).
 * `ProbabilisticScoringParameters::liquidity_penalty_multiplier_msat`'s default
   value is now 40,000 msats instead of 10,000 msats (#1375).
 * The `lightning` crate has a `grind_signatures` feature used to produce
   signatures with low r-values for more predictable transaction weight. This
   feature is on by default (#1388).
 * `ProbabilisticScoringParameters` now has a `amount_penalty_multiplier_msat`
   option, which is used to further penalize large amounts (#1399).
 * `PhantomRouteHints`, `FixedPenaltyScorer`, and `ScoringParameters` now
   implement `Clone` (#1346).

## Bug Fixes
 * Fixed a compilation error in `ProbabilisticScorer` under `--feature=no-std`
   (#1347).
 * Invoice creation functions in `lightning-invoice` crate's `utils` module
   filter invoice hints in order to limit the invoice size (#1325).
 * Fixed a bug where a `funding_locked` message was delayed by a block if the
   funding transaction was confirmed while offline, depending on the ordering
   of `Confirm::transactions_confirmed` calls when brought back online (#1363).
 * Fixed a bug in `NetGraphMsgHandler` where it didn't continue to receive
   gossip messages from peers after initial connection (#1368, #1382).
 * `ChannelManager::timer_tick_occurred` will now timeout a received multi-path
   payment (MPP) after three ticks if not received in full instead of waiting
   until near the HTLC timeout block(#1353).
 * Fixed an issue with `find_route` causing it to be overly aggressive in using
   MPP over channels to the same first hop (#1370).
 * Reduced time spent processing `channel_update` messages by checking
   signatures after checking if no newer messages have already been processed
   (#1380).
 * Fixed a few issues in `find_route` which caused preferring paths with a
   higher cost (#1398).
 * Fixed an issue in `ProbabilisticScorer` where a channel with not enough
   liquidity could still be used when retrying a failed payment if it was on a
   path with an overall lower cost (#1399).

## Serialization Compatibility
 * Channels open with `option_scid_alias` negotiated will be incompatible with
   prior releases (#1351). This may occur in the following cases:
   * Outbound channels when `ChannelHandshakeConfig::negotiate_scid_privacy` is
     enabled.
   * Inbound channels when automatically accepted from an `OpenChannel` message
     with a `channel_type` that has `ChannelTypeFeatures::supports_scid_privacy`
     return true. See `UserConfig::accept_inbound_channels`.
   * Inbound channels when manually accepted from an `OpenChannelRequest` with a
     `channel_type` that has `ChannelTypeFeatures::supports_scid_privacy` return
     true. See `UserConfig::manually_accept_inbound_channels`.

In total, this release features 43 files changed, 4052 insertions, 1274
deletions in 75 commits from 11 authors, in alphabetical order:
 * Devrandom
 * Duncan Dean
 * Elias Rohrer
 * Jeffrey Czyz
 * Jurvis Tan
 * Luiz Parreira
 * Matt Corallo
 * Omar Shamardy
 * Viktor Tigerström
 * dependabot[bot]
 * psycho-pirate


# 0.0.105 - 2022-02-28

## API Updates
 * `Phantom node` payments are now supported, allowing receipt of a payment on
   any one of multiple nodes without any coordination across the nodes being
   required. See the new `PhantomKeysManager`'s docs for more, as well as
   requirements on `KeysInterface::get_inbound_payment_key_material` and
   `lightning_invoice::utils::create_phantom_invoice` (#1199).
 * In order to support phantom node payments, several `KeysInterface` methods
   now accept a `Recipient` parameter to select between the local `node_id` and
   a phantom-specific one.
 * `ProbabilisticScorer`, a `Score` based on learning the current balances of
   channels in the network, was added. It attempts to better capture payment
   success probability than the existing `Scorer`, though may underperform on
   nodes with low payment volume. We welcome feedback on performance (#1227).
 * `Score::channel_penalty_msat` now always takes the channel value, instead of
   an `Option` (#1227).
 * `UserConfig::manually_accept_inbound_channels` was added which, when set,
   generates a new `Event::OpenChannelRequest`, which allows manual acceptance
   or rejection of incoming channels on a per-channel basis (#1281).
 * `Payee` has been renamed to `PaymentParameters` (#1271).
 * `PaymentParameters` now has a `max_total_cltv_expiry_delta` field. This
   defaults to 1008 and limits the maximum amount of time an HTLC can be pending
   before it will either fail or be claimed (#1234).
 * The `lightning-invoice` crate now supports no-std environments. This required
   numerous API changes around timestamp handling and std+no-std versions of
   several methods that previously assumed knowledge of the time (#1223, #1230).
 * `lightning-invoice` now supports parsing invoices with expiry times of more
   than one year. This required changing the semantics of `ExpiryTime` (#1273).
 * The `CounterpartyCommitmentSecrets` is now public, allowing external uses of
   the `BOLT 3` secret storage scheme (#1299).
 * Several `Sign` methods now receive HTLC preimages as proof of state
   transition, see new documentation for more (#1251).
 * `KeysInterface::sign_invoice` now provides the HRP and other invoice data
   separately to make it simpler for external signers to parse (#1272).
 * `Sign::sign_channel_announcement` now returns both the node's signature and
   the per-channel signature. `InMemorySigner` now requires the node's secret
   key in order to implement this (#1179).
 * `ChannelManager` deserialization will now fail if the `KeysInterface` used
   has a different `node_id` than the `ChannelManager` expects (#1250).
 * A new `ErrorAction` variant was added to send `warning` messages (#1013).
 * Several references to `chain::Listen` objects in `lightning-block-sync` no
   longer require a mutable reference (#1304).

## Bug Fixes
 * Fixed a regression introduced in 0.0.104 where `ChannelManager`'s internal
   locks could have an order violation leading to a deadlock (#1238).
 * Fixed cases where slow code (including user I/O) could cause us to
   disconnect peers with ping timeouts in `BackgroundProcessor` (#1269).
 * Now persist the `ChannelManager` prior to `BackgroundProcessor` stopping,
   preventing race conditions where channels are closed on startup even with a
   clean shutdown. This requires that users stop network processing and
   disconnect peers prior to `BackgroundProcessor` shutdown (#1253).
 * Fields in `ChannelHandshakeLimits` provided via the `override_config` to
   `create_channel` are now applied instead of the default config (#1292).
 * Fixed the generation of documentation on docs.rs to include API surfaces
   which are hidden behind feature flags (#1303).
 * Added the `channel_type` field to `accept_channel` messages we send, which
   may avoid some future compatibility issues with other nodes (#1314).
 * Fixed a bug where, if a previous LDK run using `lightning-persister` crashed
   while persisting updated data, we may have failed to initialize (#1332).
 * Fixed a rare bug where having both pending inbound and outbound HTLCs on a
   just-opened inbound channel could cause `ChannelDetails::balance_msat` to
   underflow and be reported as large, or cause panics in debug mode (#1268).
 * Moved more instances of verbose gossip logging from the `Trace` level to the
   `Gossip` level (#1220).
 * Delayed `announcement_signatures` until the channel has six confirmations,
   slightly improving propagation of channel announcements (#1179).
 * Several fixes in script and transaction weight calculations when anchor
   outputs are enabled (#1229).

## Serialization Compatibility
 * Using `ChannelManager` data written by versions prior to 0.0.105 will result
   in preimages for HTLCs that were pending at startup to be missing in calls
   to `KeysInterface` methods (#1251).
 * Any phantom invoice payments received on a node that is not upgraded to
   0.0.105 will fail with an "unknown channel" error. Further, downgrading to
   0.0.104 or before and then upgrading again will invalidate existing phantom
   SCIDs which may be included in invoices (#1199).

## Security
0.0.105 fixes two denial-of-service vulnerabilities which may be reachable from
untrusted input in certain application designs.

 * Route calculation spuriously panics when a routing decision is made for a
   path where the second-to-last hop is a private channel, included due to a
   multi-hop route hint in an invoice.
 * `ChannelMonitor::get_claimable_balances` spuriously panics in some scenarios
   when the LDK application's local commitment transaction is confirmed while
   HTLCs are still pending resolution.

In total, this release features 109 files changed, 7270 insertions, 2131
deletions in 108 commits from 15 authors, in alphabetical order:
 * Conor Okus
 * Devrandom
 * Elias Rohrer
 * Jeffrey Czyz
 * Jurvis Tan
 * Ken Sedgwick
 * Matt Corallo
 * Naveen
 * Tibo-lg
 * Valentine Wallace
 * Viktor Tigerström
 * dependabot[bot]
 * hackerrdave
 * naveen
 * vss96


# 0.0.104 - 2021-12-17

## API Updates
 * A `PaymentFailed` event is now provided to indicate a payment has failed
   fully. This event is generated either after
   `ChannelManager::abandon_payment` is called for a given payment, or the
   payment times out, and there are no further pending HTLCs for the payment.
   This event should be used to detect payment failure instead of
   `PaymentPathFailed::all_paths_failed`, unless no payment retries occur via
   `ChannelManager::retry_payment` (#1202).
 * Payment secrets are now generated deterministically using material from
   the new `KeysInterface::get_inbound_payment_key_material` (#1177).
 * A `PaymentPathSuccessful` event has been added to ease passing success info
   to a scorer, along with a `Score::payment_path_successful` method to accept
   such info (#1178, #1197).
 * `Score::channel_penalty_msat` has additional arguments describing the
   channel's capacity and the HTLC amount being sent over the channel (#1166).
 * A new log level `Gossip` has been added, which is used for verbose
   information generated during network graph sync. Enabling the
   `max_level_trace` feature or ignoring `Gossip` log entries reduces log
   growth during initial start up from many GiB to several MiB (#1145).
 * The `allow_wallclock_use` feature has been removed in favor of only using
   the `std` and `no-std` features (#1212).
 * `NetworkGraph` can now remove channels that we haven't heard updates for in
   two weeks with `NetworkGraph::remove_stale_channels{,with_time}`. The first
   is called automatically if a `NetGraphMsgHandler` is passed to
   `BackgroundProcessor::start` (#1212).
 * `InvoicePayer::pay_pubkey` was added to enable sending "keysend" payments to
   supported recipients, using the `InvoicePayer` to handle retires (#1160).
 * `user_payment_id` has been removed from `PaymentPurpose`, and
   `ChannelManager::create_inbound_payment{,_for_hash}` (#1180).
 * Updated documentation for several `ChannelManager` functions to remove stale
   references to panics which no longer occur (#1201).
 * The `Score` and `LockableScore` objects have moved into the
   `routing::scoring` module instead of being in the `routing` module (#1166).
 * The `Time` parameter to `ScorerWithTime` is no longer longer exposed,
   instead being fixed based on the `std`/`no-std` feature (#1184).
 * `ChannelDetails::balance_msat` was added to fetch a channel's balance
   without subtracting the reserve values, lining up with on-chain claim amounts
   less on-chain fees (#1203).
 * An explicit `UserConfig::accept_inbound_channels` flag is now provided,
   removing the need to set `min_funding_satoshis` to > 21 million BTC (#1173).
 * Inbound channels that fail to see the funding transaction confirm within
   2016 blocks are automatically force-closed with
   `ClosureReason::FundingTimedOut` (#1083).
 * We now accept a channel_reserve value of 0 from counterparties, as it is
   insecure for our counterparty but not us (#1163).
 * `NetAddress::OnionV2` parsing was removed as version 2 onion services are no
   longer supported in modern Tor (#1204).
 * Generation and signing of anchor outputs is now supported in the
   `KeysInterface`, though no support for them exists in the channel itself (#1176)

## Bug Fixes
 * Fixed a race condition in `InvoicePayer` where paths may be retried after
   the retry count has been exceeded. In this case the
   `Event::PaymentPathFailed::all_paths_failed` field is not a reliable payment
   failure indicator. There was no acceptable alternative indicator,
   `Event::PaymentFailed` as been added to provide one (#1202).
 * Reduced the blocks-before-timeout we expect of outgoing HTLCs before
   refusing to forward. This check was overly strict and resulted in refusing
   to forward som HTLCs to a next hop that had a lower security threshold than
   us (#1119).
 * LDK no longer attempt to update the channel fee for outbound channels when
   we cannot afford the new fee. This could have caused force-closure by our
   channel counterparty (#1054).
 * Fixed several bugs which may have prevented the reliable broadcast of our
   own channel announcements and updates (#1169).
 * Fixed a rare bug which may have resulted in spurious route finding failures
   when using last-hop hints and MPP with large value payments (#1168).
 * `KeysManager::spend_spendable_outputs` no longer adds a change output that
   is below the dust threshold for non-standard change scripts (#1131).
 * Fixed a minor memory leak when attempting to send a payment that fails due
   to an error when updating the `ChannelMonitor` (#1143).
 * Fixed a bug where a `FeeEstimator` that returns values rounded to the next
   sat/vbyte may result in force-closures (#1208).
 * Handle MPP timeout HTLC error codes, instead of considering the recipient to
   have sent an invalid error, removing them from the network graph (#1148)

## Serialization Compatibility
 * All above new events/fields are ignored by prior clients. All above new
   events/fields are not present when reading objects serialized by prior
   versions of the library.
 * Payment secrets are now generated deterministically. This reduces the memory
   footprint for inbound payments, however, newly-generated inbound payments
   using `ChannelManager::create_inbound_payment{,_for_hash}` will not be
   receivable using versions prior to 0.0.104.
   `ChannelManager::create_inbound_payment{,_for_hash}_legacy` are provided for
   backwards compatibility (#1177).
 * `PaymentPurpose::InvoicePayment::user_payment_id` will be 0 when reading
   objects written with 0.0.104 when read by 0.0.103 and previous (#1180).

In total, this release features 51 files changed, 5356 insertions, 2238
deletions in 107 commits from 9 authors, in alphabetical order:
 * Antoine Riard
 * Conor Okus
 * Devrandom
 * Duncan Dean
 * Elias Rohrer
 * Jeffrey Czyz
 * Ken Sedgwick
 * Matt Corallo
 * Valentine Wallace


# 0.0.103 - 2021-11-02

## API Updates
 * This release is almost entirely focused on a new API in the
   `lightning-invoice` crate - the `InvoicePayer`. `InvoicePayer` is a
   struct which takes a reference to a `ChannelManager` and a `Router`
   and retries payments as paths fail. It limits retries to a configurable
   number, but is not serialized to disk and may retry additional times across
   a serialization/load. In order to learn about failed payments, it must
   receive `Event`s directly from the `ChannelManager`, wrapping a
   user-provided `EventHandler` which it provides all unhandled events to
   (#1059).
 * `get_route` has been renamed `find_route` (#1059) and now takes a
   `RouteParameters` struct in replacement of a number of its long list of
   arguments (#1134). The `Payee` in the `RouteParameters` is stored in the
   `Route` object returned and provided in the `RouteParameters` contained in
   `Event::PaymentPathFailed` (#1059).
 * `ChannelMonitor`s must now be persisted after calls that provide new block
   data, prior to `MonitorEvent`s being passed back to `ChannelManager` for
   processing. If you are using a `ChainMonitor` this is handled for you.
   The `Persist` API has been updated to `Option`ally take the
   `ChannelMonitorUpdate` as persistence events that result from chain data no
   longer have a corresponding update (#1108).
 * `routing::Score` now has a `payment_path_failed` method which it can use to
   learn which channels often fail payments. It is automatically called by
   `InvoicePayer` for failed payment paths (#1144).
 * The default `Scorer` implementation is now a type alias to a type generic
   across different clocks and supports serialization to persist scoring data
   across restarts (#1146).
 * `Event::PaymentSent` now includes the full fee which was spent across all
   payment paths which were fulfilled or pending when the payment was fulfilled
   (#1142).
 * `Event::PaymentSent` and `Event::PaymentPathFailed` now include the
   `PaymentId` which matches the `PaymentId` returned from
   `ChannelManager::send_payment` or `InvoicePayer::pay_invoice` (#1059).
 * `NetGraphMsgHandler` now takes a `Deref` to the `NetworkGraph`, allowing for
   shared references to the graph data to make serialization and references to
   the graph data in the `InvoicePayer`'s `Router` simpler (#1149).
 * `routing::Score::channel_penalty_msat` has been updated to provide the
   `NodeId` of both the source and destination nodes of a channel (#1133).

## Bug Fixes
 * Previous versions would often disconnect peers during initial graph sync due
   to ping timeouts while processing large numbers of gossip messages. We now
   delay disconnecting peers if we receive messages from them even if it takes
   a while to receive a pong from them. Further, we avoid sending too many
   gossip messages between pings to ensure we should always receive pongs in a
   timely manner (#1137).
 * If a payment was sent, creating an outbound HTLC and sending it to our
   counterparty (implying the `ChannelMonitor` was persisted on disk), but the
   `ChannelManager` was not persisted prior to shutdown/crash, no
   `Event::PaymentPathFailed` event was generated if the HTLC was eventually
   failed on chain. Events are now consistent irrespective of `ChannelManager`
   persistence or non-persistence (#1104).

## Serialization Compatibility
 * All above new Events/fields are ignored by prior clients. All above new
   Events/fields are not present when reading objects serialized by prior
   versions of the library.
 * Payments for which a `Route` was generated using a previous version or for
   which the payment was originally sent by a previous version of the library
   will not be retried by an `InvoicePayer`.

This release was singularly focused and some contributions by third parties
were delayed.
In total, this release features 38 files changed, 4414 insertions, and 969
deletions in 71 commits from 2 authors, in alphabetical order:

 * Jeffrey Czyz
 * Matt Corallo


# 0.0.102 - 2021-10-18

## API Updates
 * `get_route` now takes a `Score` as an argument. `Score` is queried during
   the route-finding process, returning the absolute amounts which you are
   willing to pay to avoid routing over a given channel. As a default, a
   `Scorer` is provided which returns a constant amount, with a suggested
   default of 500 msat. This translates to a willingness to pay up to 500 msat
   in additional fees per hop in order to avoid additional hops (#1124).
 * `Event::PaymentPathFailed` now contains a `short_channel_id` field which may
   be filled in with a channel that can be "blamed" for the payment failure.
   Payment retries should likely avoid the given channel for some time (#1077).
 * `PublicKey`s in `NetworkGraph` have been replaced with a `NodeId` struct
   which contains only a simple `[u8; 33]`, substantially improving
   `NetworkGraph` deserialization performance (#1107).
 * `ChainMonitor`'s `HashMap` of `ChannelMonitor`s is now private, exposed via
   `Chainmonitor::get_monitor` and `ChainMonitor::list_monitors` instead
   (#1112).
 * When an outbound channel is closed prior to the broadcasting of its funding
   transaction, but after you call
   `ChannelManager::funding_transaction_generated`, a new event type,
   `Event::DiscardFunding`, is generated, informing you the transaction was not
   broadcasted and that you can spend the same inputs again elsewhere (#1098).
 * `ChannelManager::create_channel` now returns the temporary channel ID which
   may later appear in `Event::ChannelClosed` or `ChannelDetails` prior to the
   channel being funded (#1121).
 * `Event::PaymentSent` now contains the payment hash as well as the payment
   preimage (#1062).
 * `ReadOnlyNetworkGraph::get_addresses` now returns owned `NetAddress` rather
   than references. As a side-effect this method is now exposed in foreign
   language bindings (#1115).
 * The `Persist` and `ChannelMonitorUpdateErr` types have moved to the
   `lightning::chain::chainmonitor` and `lightning::chain` modules,
   respectively (#1112).
 * `ChannelManager::send_payment` now returns a `PaymentId` which identifies a
   payment (whether MPP or not) and can be used to retry the full payment or
   MPP parts through `retry_payment` (#1096). Note that doing so is currently
   *not* crash safe, and you may find yourself sending twice. It is recommended
   that you *not* use the `retry_payment` API until the next release.

## Bug Fixes
 * Due to an earlier fix for the Lightning dust inflation vulnerability tracked
   in CVE-2021-41591/CVE-2021-41592/CVE-2021-41593 in 0.0.100, we required
   counterparties to accept a dust limit slightly lower than the dust limit now
   required by other implementations. This appeared as, at least, latest lnd
   always refusing to accept channels opened by LDK clients (#1065).
 * If there are multiple channels available to the same counterparty,
   `get_route` would only consider the channel listed last as available for
   sending (#1100).
 * `Persist` implementations returning
   `ChannelMonitorUpdateErr::TemporaryFailure` from `watch_channel` previously
   resulted in the `ChannelMonitor` not being stored at all, resulting in a
   panic after monitor updating is complete (#1112).
 * If payments are pending awaiting forwarding at startup, an
   `Event::PendingHTLCsForwardable` event will always be provided. This ensures
   user code calls `ChannelManager::process_pending_htlc_fowards` even if it
   shut down while awaiting the batching timer during the previous run (#1076).
 * If a call to `ChannelManager::send_payment` failed due to lack of
   availability of funds locally, LDK would store the payment as pending
   forever, with no ability to retry or fail it, leaking memory (#1109).

## Serialization Compatibility
 * All above new Events/fields are ignored by prior clients. All above new
   Events/fields, except for `Event::PaymentSent::payment_hash` are not present
   when reading objects serialized by prior versions of the library.

In total, this release features 32 files changed, 2248 insertions, and 1483
deletions in 51 commits from 7 authors, in alphabetical order:

 * 1nF0rmed
 * Duncan Dean
 * Elias Rohrer
 * Galder Zamarreño
 * Jeffrey Czyz
 * Matt Corallo
 * Valentine Wallace


# 0.0.101 - 2021-09-23

## API Updates
 * Custom message types are now supported directly in the `PeerManager`,
   allowing you to send and receive messages of any type that is not natively
   understood by LDK. This requires a new type bound on `PeerManager`, a
   `CustomMessageHandler`. `IgnoringMessageHandler` provides a simple default
   for this new bound for ignoring unknown messages (#1031, #1074).
 * Route graph updates as a result of failed payments are no longer provided as
   `MessageSendEvent::PaymentFailureNetworkUpdate` but instead included in a
   new field in the `Event::PaymentFailed` events. Generally, this means route
   graph updates are no longer handled as a part of the `PeerManager` but
   instead through the new `EventHandler` implementation for
   `NetGraphMsgHandler`. To make this easy, a new parameter to
   `lightning-background-processor::BackgroundProcessor::start` is added, which
   contains an `Option`al `NetGraphmsgHandler`. If provided as `Some`, relevant
   events will be processed by the `NetGraphMsgHandler` prior to normal event
   handling (#1043).
 * `NetworkGraph` is now, itself, thread-safe. Accordingly, most functions now
   take `&self` instead of `&mut self` and the graph data can be accessed
   through `NetworkGraph.read_only` (#1043).
 * The balances available on-chain to claim after a channel has been closed are
   now exposed via `ChannelMonitor::get_claimable_balances` and
   `ChainMonitor::get_claimable_balances`. The second can be used to get
   information about all closed channels which still have on-chain balances
   associated with them. See enum variants of `ln::channelmonitor::Balance` and
   method documentation for the above methods for more information on the types
   of balances exposed (#1034).
 * When one HTLC of a multi-path payment fails, the new field `all_paths_failed`
   in `Event::PaymentFailed` is set to `false`. This implies that the payment
   has not failed, but only one part. Payment resolution is only indicated by an
   `Event::PaymentSent` event or an `Event::PaymentFailed` with
   `all_paths_failed` set to `true`, which is also set for the last remaining
   part of a multi-path payment (#1053).
 * To better capture the context described above, `Event::PaymentFailed` has
   been renamed to `Event::PaymentPathFailed` (#1084).
 * A new event, `ChannelClosed`, is provided by `ChannelManager` when a channel
   is closed, including a reason and error message (if relevant, #997).
 * `lightning-invoice` now considers invoices with sub-millisatoshi precision
   to be invalid, and requires millisatoshi values during construction (thus
   you must call `amount_milli_satoshis` instead of `amount_pico_btc`, #1057).
 * The `BaseSign` interface now includes two new hooks which provide additional
   information about commitment transaction signatures and revocation secrets
   provided by our counterparty, allowing additional verification (#1039).
 * The `BaseSign` interface now includes additional information for cooperative
   close transactions, making it easier for a signer to verify requests (#1064).
 * `Route` has two additional helper methods to get fees and amounts (#1063).
 * `Txid` and `Transaction` objects can now be deserialized from responses when
   using the HTTP client in the `lightning-block-sync` crate (#1037, #1061).

## Bug Fixes
 * Fix a panic when reading a lightning invoice with a non-recoverable
   signature. Further, restrict lightning invoice parsing to require payment
   secrets and better handle a few edge cases as required by BOLT 11 (#1057).
 * Fix a panic when receiving multiple messages (such as HTLC fulfill messages)
   after a call to `chain::Watch::update_channel` returned
   `Err(ChannelMonitorUpdateErr::TemporaryFailure)` with no
   `ChannelManager::channel_monitor_updated` call in between (#1066).
 * For multi-path payments, `Event::PaymentSent` is no longer generated
   multiple times, once for each independent part (#1053).
 * Multi-hop route hints in invoices are now considered in the default router
   provided via `get_route` (#1040).
 * The time peers have to respond to pings has been increased when building
   with debug assertions enabled. This avoids peer disconnections on slow hosts
   when running in debug mode (#1051).
 * The timeout for the first byte of a response for requests from the
   `lightning-block-sync` crate has been increased to 300 seconds to better
   handle the long hangs in Bitcoin Core when it syncs to disk (#1090).

## Serialization Compatibility
 * Due to a bug in 0.0.100, `Event`s written by 0.0.101 which are of a type not
   understood by 0.0.100 may lead to `Err(DecodeError::InvalidValue)` or corrupt
   deserialized objects in 0.100. Such `Event`s will lead to an
   `Err(DecodeError::InvalidValue)` in versions prior to 0.0.100. The only such
   new event written by 0.0.101 is `Event::ChannelClosed` (#1087).
 * Payments that were initiated in versions prior to 0.0.101 may still
   generate duplicate `PaymentSent` `Event`s or may have spurious values for
   `Event::PaymentPathFailed::all_paths_failed` (#1053).
 * The return values of `ChannelMonitor::get_claimable_balances` (and, thus,
   `ChainMonitor::get_claimable_balances`) may be spurious for channels where
   the spend of the funding transaction appeared on chain while running a
   version prior to 0.0.101. `Balance` information should only be relied upon
   for channels that were closed while running 0.0.101+ (#1034).
 * Payments failed while running versions prior to 0.0.101 will never have a
   `Some` for the `network_update` field (#1043).

In total, this release features 67 files changed, 4980 insertions, 1888
deletions in 89 commits from 12 authors, in alphabetical order:
 * Antoine Riard
 * Devrandom
 * Galder Zamarreño
 * Giles Cope
 * Jeffrey Czyz
 * Joseph Goulden
 * Matt Corallo
 * Sergi Delgado Segura
 * Tibo-lg
 * Valentine Wallace
 * abhik-99
 * vss96


# 0.0.100 - 2021-08-17 - "Oh, so *that's* what's going on inside the box"

## API Updates
 * The `lightning` crate can now be built in no_std mode, making it easy to
   target embedded hardware for rust users. Note that mutexes are replaced with
   no-ops for such builds (#1008, #1028).
 * LDK now supports sending and receiving "keysend" payments. This includes
   modifications to `lightning::util::events::Event::PaymentReceived` to
   indicate the type of payment (#967).
 * A new variant, `lightning::util::events::Event::PaymentForwarded` has been
   added which indicates a forwarded payment has been successfully claimed and
   we've received a forwarding fee (#1004).
 * `lightning::chain::keysinterface::KeysInterface::get_shutdown_pubkey` has
   been renamed to `get_shutdown_scriptpubkey`, returns a script, and is now
   called on channel open only if
   `lightning::util::config::ChannelConfig::commit_upfront_shutdown_pubkey` is
   set (#1019).
 * Closing-signed negotiation is now more configurable, with an explicit
   `lightning::util::config::ChannelConfig::force_close_avoidance_max_fee_satoshis`
   field allowing you to select the maximum amount you are willing to pay to
   avoid a force-closure. Further, we are now less restrictive on the fee
   placed on the closing transaction when we are not the party paying it. To
   control the feerate paid on a channel at close-time, use
   `ChannelManager::close_channel_with_target_feerate` instead of
   `close_channel` (#1011).
 * `lightning_background_processor::BackgroundProcessor` now stops the
   background thread when dropped (#1007). It is marked `#[must_use]` so that
   Rust users will receive a compile-time warning when it is immediately
   dropped after construction (#1029).
 * Total potential funds burn on force-close due to dust outputs is now limited
   to `lightning::util::config::ChannelConfig::max_dust_htlc_exposure_msat` per
   channel (#1009).
 * The interval on which
   `lightning::ln::peer_handler::PeerManager::timer_tick_occurred` should be
   called has been reduced to once every five seconds (#1035) and
   `lightning::ln::channelmanager::ChannelManager::timer_tick_occurred` should
   now be called on startup in addition to once per minute (#985).
 * The rust-bitcoin and bech32 dependencies have been updated to their
   respective latest versions (0.27 and 0.8, #1012).

## Bug Fixes
 * Fix panic when reading invoices generated by some versions of c-lightning
   (#1002 and #1003).
 * Fix panic when attempting to validate a signed message of incorrect length
   (#1010).
 * Do not ignore the route hints in invoices when the invoice is over 250k
   sats (#986).
 * Fees are automatically updated on outbound channels to ensure commitment
   transactions are always broadcastable (#985).
 * Fixes a rare case where a `lightning::util::events::Event::SpendableOutputs`
   event is not generated after a counterparty commitment transaction is
   confirmed in a reorg when a conflicting local commitment transaction is
   removed in the same reorg (#1022).
 * Fixes a remotely-triggerable force-closure of an origin channel after an
   HTLC was forwarded over a next-hop channel and the next-hop channel was
   force-closed by our counterparty (#1025).
 * Fixes a rare force-closure case when sending a payment as a channel fundee
   when overdrawing our remaining balance. Instead the send will fail (#998).
 * Fixes a rare force-closure case when a payment was claimed prior to a
   peer disconnection or restart, and later failed (#977).

## Serialization Compatibility
 * Pending inbound keysend payments which have neither been failed nor claimed
   when serialized will result in a `ChannelManager` which is not readable on
   pre-0.0.100 clients (#967).
 * Because
   `lightning::chain::keysinterface::KeysInterface::get_shutdown_scriptpubkey`
   has been updated to return a script instead of only a `PublicKey`,
   `ChannelManager`s constructed with custom `KeysInterface` implementations on
   0.0.100 and later versions will not be readable on previous versions.
   `ChannelManager`s created with 0.0.99 and prior versions will remain readable
   even after the a serialization roundtrip on 0.0.100, as long as no new
   channels are opened. Further, users using a
   `lightning::chain::keysinterface::KeysManager` as their `KeysInterface` will
   have `ChannelManager`s which are readable on prior versions as well (#1019).
 * `ChannelMonitorUpdate`s created by 0.0.100 and later for channels when
   `lightning::util::config::ChannelConfig::commit_upfront_shutdown_pubkey` is
   not set may not be readable by versions prior to 0.0.100 (#1019).
 * HTLCs which were in the process of being claimed on-chain when a pre-0.0.100
   `ChannelMonitor` was serialized may generate `PaymentForwarded` events with
   spurious `fee_earned_msat` values. This only applies to payments which were
   unresolved at the time of the upgrade (#1004).
 * 0.0.100 clients with pending `Event::PaymentForwarded` events at
   serialization-time will generate serialized `ChannelManager` objects which
   0.0.99 and earlier clients cannot read. The likelihood of this can be reduced
   by ensuring you process all pending events immediately before serialization
   (as is done by the `lightning-background-processor` crate, #1004).


In total, this release features 59 files changed, 5861 insertions, and 2082
deletions in 95 commits from 6 authors.


# 0.0.99 - 2021-07-09 - "It's a Bugz Life"

## API Updates

 * `lightning_block_sync::poll::Validate` is now public, allowing you to
   implement the `lightning_block_sync::poll::Poll` trait without
   `lightning_block_sync::poll::ChainPoller` (#956).
 * `lightning::ln::peer_handler::PeerManager` no longer requires that no calls
   are made to referencing the same `SocketDescriptor` after
   `disconnect_socket` returns. This makes the API significantly less
   deadlock-prone and simplifies `SocketDescriptor` implementations
   significantly. The relevant changes have been made to `lightning_net_tokio`
   and `PeerManager` documentation has been substantially rewritten (#957).
 * `lightning::util::message_signing`'s `sign` and `verify` methods now take
   secret and public keys by reference instead of value (#974).
 * Substantially more information is now exposed about channels in
   `ChannelDetails`. See documentation for more info (#984 and #988).
 * The latest best block seen is now exposed in
   `ChannelManager::current_best_block` and
   `ChannelMonitor::current_best_block` (#984).
 * Feerates charged when forwarding payments over channels is now set in
   `ChannelConfig::fee_base_msat` when the channel is opened. For existing
   channels, the value is set to the value provided in
   `ChannelManagerReadArgs::default_config::channel_options` the first time the
   `ChannelManager` is loaded in 0.0.99 (#975).
 * We now reject HTLCs which are received to be forwarded over private channels
   unless `UserConfig::accept_forwards_to_priv_channels` is set. Note that
   `UserConfig` is never serialized and must be provided via
   `ChannelManagerReadArgs::default_config` at each start (#975).

## Bug Fixes

 * We now forward gossip messages to peers instead of only relaying
   locally-generated gossip or sending gossip messages during initial sync
   (#948).
 * Correctly send `channel_update` messages to direct peers on private channels
   (#949). Without this, a private node connected to an LDK node over a private
   channel cannot receive funds as it does not know which fees the LDK node
   will charge.
 * `lightning::ln::channelmanager::ChannelManager` no longer expects to be
   persisted spuriously after we receive a `channel_update` message about any
   channel in the routing gossip (#972).
 * Asynchronous `ChannelMonitor` updates (using the
   `ChannelMonitorUpdateErr::TemporaryFailure` return variant) no longer cause
   spurious HTLC forwarding failures (#954).
 * Transaction provided via `ChannelMonitor::transactions_confirmed`
   after `ChannelMonitor::best_block_updated` was called for a much later
   block now trigger all relevant actions as of the later block. Previously
   some transaction broadcasts or other responses required an additional
   block be provided via `ChannelMonitor::best_block_updated` (#970).
 * We no longer panic in rare cases when an invoice contained last-hop route
   hints which were unusable (#958).

## Node Compatibility

 * We now accept spurious `funding_locked` messages sent prior to
   `channel_reestablish` messages after reconnect. This is a
   [known, long-standing bug in lnd](https://github.com/lightningnetwork/lnd/issues/4006)
   (#966).
 * We now set the `first_blocknum` and `number_of_blocks` fields in
   `reply_channel_range` messages to values which c-lightning versions prior to
   0.10 accepted. This avoids spurious force-closes from such nodes (#961).

## Serialization Compatibility

 * Due to a bug discovered in 0.0.98, if a `ChannelManager` is serialized on
   version 0.0.98 while an `Event::PaymentSent` is pending processing, the
   `ChannelManager` will fail to deserialize both on version 0.0.98 and later
   versions. If you have such a `ChannelManager` available, a simple patch will
   allow it to deserialize. Please file an issue if you need assistance (#973).

# 0.0.98 - 2021-06-11 - "It's ALIVVVVEEEEEEE"

0.0.98 should be considered a release candidate to the first alpha release of
Rust-Lightning and the broader LDK. It represents several years of work
designing and fine-tuning a flexible API for integrating lightning into any
application. LDK should make it easy to build a lightning node or client which
meets specific requirements that other lightning node software cannot. As
lightning continues to evolve, and new use-cases for lightning develop, the API
of LDK will continue to change and expand. However, starting with version 0.1,
objects serialized with prior versions will be readable with the latest LDK.
While Rust-Lightning is approaching the 0.1 milestone, language bindings
components of LDK available at https://github.com/lightningdevkit are still of
varying quality. Some are also approaching an 0.1 release, while others are
still much more experimental. Please note that, at 0.0.98, using Rust-Lightning
on mainnet is *strongly* discouraged.
