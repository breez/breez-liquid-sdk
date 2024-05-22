// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.35.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import 'frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

class ConnectRequest {
  final String mnemonic;
  final String? dataDir;
  final Network network;

  const ConnectRequest({
    required this.mnemonic,
    this.dataDir,
    required this.network,
  });

  @override
  int get hashCode => mnemonic.hashCode ^ dataDir.hashCode ^ network.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is ConnectRequest &&
          runtimeType == other.runtimeType &&
          mnemonic == other.mnemonic &&
          dataDir == other.dataDir &&
          network == other.network;
}

class GetInfoRequest {
  final bool withScan;

  const GetInfoRequest({
    required this.withScan,
  });

  @override
  int get hashCode => withScan.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is GetInfoRequest && runtimeType == other.runtimeType && withScan == other.withScan;
}

class GetInfoResponse {
  /// Usable balance. This is the confirmed onchain balance minus `pending_send_sat`.
  final int balanceSat;

  /// Amount that is being used for ongoing Send swaps
  final int pendingSendSat;

  /// Incoming amount that is pending from ongoing Receive swaps
  final int pendingReceiveSat;
  final String pubkey;

  const GetInfoResponse({
    required this.balanceSat,
    required this.pendingSendSat,
    required this.pendingReceiveSat,
    required this.pubkey,
  });

  @override
  int get hashCode =>
      balanceSat.hashCode ^ pendingSendSat.hashCode ^ pendingReceiveSat.hashCode ^ pubkey.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is GetInfoResponse &&
          runtimeType == other.runtimeType &&
          balanceSat == other.balanceSat &&
          pendingSendSat == other.pendingSendSat &&
          pendingReceiveSat == other.pendingReceiveSat &&
          pubkey == other.pubkey;
}

enum Network {
  liquid,
  liquidTestnet,
  ;
}

enum NewSwapState {
  created,
  pending,
  complete,
  failed,
  ;
}

/// Represents an SDK payment.
///
/// By default, this is an onchain tx. It may represent a swap, if swap metadata is available.
class Payment {
  /// The tx ID of the onchain transaction
  final String txId;

  /// The swap ID, if any swap is associated with this payment
  final String? swapId;

  /// Composite timestamp that can be used for sorting or displaying the payment.
  ///
  /// If this payment has an associated swap, it is the swap creation time. Otherwise, the point
  /// in time when the underlying tx was included in a block. If there is no associated swap
  /// available and the underlying tx is not yet confirmed, the value is `now()`.
  final int timestamp;

  /// The payment amount, which corresponds to the onchain tx amount.
  ///
  /// In case of an outbound payment (Send), this is the payer amount. Otherwise it's the receiver amount.
  final int amountSat;

  /// If a swap is associated with this payment, this represents the total fees paid by the
  /// sender. In other words, it's the delta between the amount that was sent and the amount
  /// received.
  final int? feesSat;
  final PaymentType paymentType;

  /// Composite status representing the overall status of the payment.
  ///
  /// If the tx has no associated swap, this reflects the onchain tx status (confirmed or not).
  ///
  /// If the tx has an associated swap, this is determined by the swap status (pending or complete).
  final NewSwapState status;

  const Payment({
    required this.txId,
    this.swapId,
    required this.timestamp,
    required this.amountSat,
    this.feesSat,
    required this.paymentType,
    required this.status,
  });

  @override
  int get hashCode =>
      txId.hashCode ^
      swapId.hashCode ^
      timestamp.hashCode ^
      amountSat.hashCode ^
      feesSat.hashCode ^
      paymentType.hashCode ^
      status.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is Payment &&
          runtimeType == other.runtimeType &&
          txId == other.txId &&
          swapId == other.swapId &&
          timestamp == other.timestamp &&
          amountSat == other.amountSat &&
          feesSat == other.feesSat &&
          paymentType == other.paymentType &&
          status == other.status;
}

enum PaymentType {
  receive,
  send,
  ;
}

class PrepareReceiveRequest {
  final int payerAmountSat;

  const PrepareReceiveRequest({
    required this.payerAmountSat,
  });

  @override
  int get hashCode => payerAmountSat.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is PrepareReceiveRequest &&
          runtimeType == other.runtimeType &&
          payerAmountSat == other.payerAmountSat;
}

class PrepareReceiveResponse {
  final int payerAmountSat;
  final int feesSat;

  const PrepareReceiveResponse({
    required this.payerAmountSat,
    required this.feesSat,
  });

  @override
  int get hashCode => payerAmountSat.hashCode ^ feesSat.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is PrepareReceiveResponse &&
          runtimeType == other.runtimeType &&
          payerAmountSat == other.payerAmountSat &&
          feesSat == other.feesSat;
}

class PrepareSendRequest {
  final String invoice;

  const PrepareSendRequest({
    required this.invoice,
  });

  @override
  int get hashCode => invoice.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is PrepareSendRequest && runtimeType == other.runtimeType && invoice == other.invoice;
}

class PrepareSendResponse {
  final String invoice;
  final int feesSat;

  const PrepareSendResponse({
    required this.invoice,
    required this.feesSat,
  });

  @override
  int get hashCode => invoice.hashCode ^ feesSat.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is PrepareSendResponse &&
          runtimeType == other.runtimeType &&
          invoice == other.invoice &&
          feesSat == other.feesSat;
}

class ReceivePaymentResponse {
  final String id;
  final String invoice;

  const ReceivePaymentResponse({
    required this.id,
    required this.invoice,
  });

  @override
  int get hashCode => id.hashCode ^ invoice.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is ReceivePaymentResponse &&
          runtimeType == other.runtimeType &&
          id == other.id &&
          invoice == other.invoice;
}

class RestoreRequest {
  final String? backupPath;

  const RestoreRequest({
    this.backupPath,
  });

  @override
  int get hashCode => backupPath.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is RestoreRequest && runtimeType == other.runtimeType && backupPath == other.backupPath;
}

class SendPaymentResponse {
  final String txid;

  const SendPaymentResponse({
    required this.txid,
  });

  @override
  int get hashCode => txid.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is SendPaymentResponse && runtimeType == other.runtimeType && txid == other.txid;
}
