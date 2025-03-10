// This file is Copyright its original authors, visible in version control
// history.
//
// This file is licensed under the Apache License, Version 2.0 <LICENSE-APACHE
// or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// You may not use this file except in accordance with one or both of these
// licenses.

//! Convenient utilities for paying Lightning invoices.

use bitcoin::hashes::Hash;
use lightning_invoice::Bolt11Invoice;

use crate::ln::channelmanager::RecipientOnionFields;
use crate::routing::router::{PaymentParameters, RouteParameters};
use crate::types::payment::PaymentHash;

/// Builds the necessary parameters to pay or pre-flight probe the given variable-amount
/// (also known as 'zero-amount') [`Bolt11Invoice`] using
/// [`ChannelManager::send_payment`] or [`ChannelManager::send_preflight_probes`].
///
/// Prior to paying, you must ensure that the [`Bolt11Invoice::payment_hash`] is unique and the
/// same [`PaymentHash`] has never been paid before.
///
/// Will always succeed unless the invoice has an amount specified, in which case
/// [`payment_parameters_from_invoice`] should be used.
///
/// [`ChannelManager::send_payment`]: crate::ln::channelmanager::ChannelManager::send_payment
/// [`ChannelManager::send_preflight_probes`]: crate::ln::channelmanager::ChannelManager::send_preflight_probes
pub fn payment_parameters_from_variable_amount_invoice(
	invoice: &Bolt11Invoice, amount_msat: u64,
) -> Result<(PaymentHash, RecipientOnionFields, RouteParameters), ()> {
	if invoice.amount_milli_satoshis().is_some() {
		Err(())
	} else {
		Ok(params_from_invoice(invoice, amount_msat))
	}
}

/// Builds the necessary parameters to pay or pre-flight probe the given [`Bolt11Invoice`] using
/// [`ChannelManager::send_payment`] or [`ChannelManager::send_preflight_probes`].
///
/// Prior to paying, you must ensure that the [`Bolt11Invoice::payment_hash`] is unique and the
/// same [`PaymentHash`] has never been paid before.
///
/// Will always succeed unless the invoice has no amount specified, in which case
/// [`payment_parameters_from_variable_amount_invoice`] should be used.
///
/// [`ChannelManager::send_payment`]: crate::ln::channelmanager::ChannelManager::send_payment
/// [`ChannelManager::send_preflight_probes`]: crate::ln::channelmanager::ChannelManager::send_preflight_probes
pub fn payment_parameters_from_invoice(
	invoice: &Bolt11Invoice,
) -> Result<(PaymentHash, RecipientOnionFields, RouteParameters), ()> {
	if let Some(amount_msat) = invoice.amount_milli_satoshis() {
		Ok(params_from_invoice(invoice, amount_msat))
	} else {
		Err(())
	}
}

fn params_from_invoice(
	invoice: &Bolt11Invoice, amount_msat: u64,
) -> (PaymentHash, RecipientOnionFields, RouteParameters) {
	let payment_hash = PaymentHash((*invoice.payment_hash()).to_byte_array());

	let mut recipient_onion = RecipientOnionFields::secret_only(*invoice.payment_secret());
	recipient_onion.payment_metadata = invoice.payment_metadata().map(|v| v.clone());

	let mut payment_params = PaymentParameters::from_node_id(
		invoice.recover_payee_pub_key(),
		invoice.min_final_cltv_expiry_delta() as u32,
	)
	.with_route_hints(invoice.route_hints())
	.unwrap();
	if let Some(expiry) = invoice.expires_at() {
		payment_params = payment_params.with_expiry_time(expiry.as_secs());
	}
	if let Some(features) = invoice.features() {
		payment_params = payment_params.with_bolt11_features(features.clone()).unwrap();
	}

	let route_params = RouteParameters::from_payment_params_and_value(payment_params, amount_msat);
	(payment_hash, recipient_onion, route_params)
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::routing::router::Payee;
	use crate::sign::{NodeSigner, Recipient};
	use crate::types::payment::PaymentSecret;
	use bitcoin::hashes::sha256::Hash as Sha256;
	use bitcoin::secp256k1::{PublicKey, Secp256k1, SecretKey};
	use lightning_invoice::{Currency, InvoiceBuilder};
	use std::time::SystemTime;

	#[test]
	fn invoice_test() {
		let payment_hash = Sha256::hash(&[0; 32]);
		let private_key = SecretKey::from_slice(&[42; 32]).unwrap();
		let secp_ctx = Secp256k1::new();
		let public_key = PublicKey::from_secret_key(&secp_ctx, &private_key);

		let timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
		let invoice = InvoiceBuilder::new(Currency::Bitcoin)
			.description("test".into())
			.payment_hash(payment_hash)
			.payment_secret(PaymentSecret([0; 32]))
			.duration_since_epoch(timestamp)
			.min_final_cltv_expiry_delta(144)
			.amount_milli_satoshis(128)
			.build_signed(|hash| secp_ctx.sign_ecdsa_recoverable(hash, &private_key))
			.unwrap();

		assert!(payment_parameters_from_variable_amount_invoice(&invoice, 42).is_err());

		let (hash, onion, params) = payment_parameters_from_invoice(&invoice).unwrap();
		assert_eq!(&hash.0[..], &payment_hash[..]);
		assert_eq!(onion.payment_secret, Some(PaymentSecret([0; 32])));
		assert_eq!(params.final_value_msat, 128);
		match params.payment_params.payee {
			Payee::Clear { node_id, .. } => {
				assert_eq!(node_id, public_key);
			},
			_ => panic!(),
		}
	}

	#[test]
	fn zero_value_invoice_test() {
		let payment_hash = Sha256::hash(&[0; 32]);
		let private_key = SecretKey::from_slice(&[42; 32]).unwrap();
		let secp_ctx = Secp256k1::new();
		let public_key = PublicKey::from_secret_key(&secp_ctx, &private_key);

		let timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
		let invoice = InvoiceBuilder::new(Currency::Bitcoin)
			.description("test".into())
			.payment_hash(payment_hash)
			.payment_secret(PaymentSecret([0; 32]))
			.duration_since_epoch(timestamp)
			.min_final_cltv_expiry_delta(144)
			.build_signed(|hash| secp_ctx.sign_ecdsa_recoverable(hash, &private_key))
			.unwrap();

		assert!(payment_parameters_from_invoice(&invoice).is_err());

		let (hash, onion, params) =
			payment_parameters_from_variable_amount_invoice(&invoice, 42).unwrap();
		assert_eq!(&hash.0[..], &payment_hash[..]);
		assert_eq!(onion.payment_secret, Some(PaymentSecret([0; 32])));
		assert_eq!(params.final_value_msat, 42);
		match params.payment_params.payee {
			Payee::Clear { node_id, .. } => {
				assert_eq!(node_id, public_key);
			},
			_ => panic!(),
		}
	}

	#[test]
	fn payment_metadata_end_to_end() {
		use crate::events::Event;
		use crate::ln::channelmanager::{PaymentId, Retry};
		use crate::ln::functional_test_utils::*;
		use crate::ln::msgs::ChannelMessageHandler;

		// Test that a payment metadata read from an invoice passed to `pay_invoice` makes it all
		// the way out through the `PaymentClaimable` event.
		let chanmon_cfgs = create_chanmon_cfgs(2);
		let node_cfgs = create_node_cfgs(2, &chanmon_cfgs);
		let node_chanmgrs = create_node_chanmgrs(2, &node_cfgs, &[None, None]);
		let nodes = create_network(2, &node_cfgs, &node_chanmgrs);
		create_announced_chan_between_nodes(&nodes, 0, 1);

		let payment_metadata = vec![42, 43, 44, 45, 46, 47, 48, 49, 42];

		let (payment_hash, payment_secret) =
			nodes[1].node.create_inbound_payment(None, 7200, None).unwrap();

		let timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
		let invoice = InvoiceBuilder::new(Currency::Bitcoin)
			.description("test".into())
			.payment_hash(Sha256::from_slice(&payment_hash.0).unwrap())
			.payment_secret(payment_secret)
			.duration_since_epoch(timestamp)
			.min_final_cltv_expiry_delta(144)
			.amount_milli_satoshis(50_000)
			.payment_metadata(payment_metadata.clone())
			.build_raw()
			.unwrap();
		let sig = nodes[1].keys_manager.backing.sign_invoice(&invoice, Recipient::Node).unwrap();
		let invoice = invoice.sign::<_, ()>(|_| Ok(sig)).unwrap();
		let invoice = Bolt11Invoice::from_signed(invoice).unwrap();

		let (hash, onion, params) = payment_parameters_from_invoice(&invoice).unwrap();
		nodes[0]
			.node
			.send_payment(hash, onion, PaymentId(hash.0), params, Retry::Attempts(0))
			.unwrap();
		check_added_monitors(&nodes[0], 1);
		let send_event = SendEvent::from_node(&nodes[0]);
		nodes[1].node.handle_update_add_htlc(nodes[0].node.get_our_node_id(), &send_event.msgs[0]);
		commitment_signed_dance!(nodes[1], nodes[0], &send_event.commitment_msg, false);

		expect_pending_htlcs_forwardable!(nodes[1]);

		let mut events = nodes[1].node.get_and_clear_pending_events();
		assert_eq!(events.len(), 1);
		match events.pop().unwrap() {
			Event::PaymentClaimable { onion_fields, .. } => {
				assert_eq!(Some(payment_metadata), onion_fields.unwrap().payment_metadata);
			},
			_ => panic!("Unexpected event"),
		}
	}
}
