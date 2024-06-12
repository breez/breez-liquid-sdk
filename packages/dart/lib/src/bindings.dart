// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.38.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import 'error.dart';
import 'frb_generated.dart';
import 'lib.dart';
import 'model.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

// These functions are ignored because they are not marked as `pub`: `init`
// These types are ignored because they are not used by any `pub` functions: `DartBindingLogger`

Future<BindingLiquidSdk> connect({required ConnectRequest req}) =>
    RustLib.instance.api.crateBindingsConnect(req: req);

/// If used, this must be called before `connect`. It can only be called once.
Stream<LogEntry> breezLogStream() => RustLib.instance.api.crateBindingsBreezLogStream();

Config defaultConfig({required LiquidSdkNetwork network}) =>
    RustLib.instance.api.crateBindingsDefaultConfig(network: network);

Future<InputType> parse({required String input}) => RustLib.instance.api.crateBindingsParse(input: input);

LNInvoice parseInvoice({required String input}) =>
    RustLib.instance.api.crateBindingsParseInvoice(input: input);

// Rust type: RustOpaqueNom<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<BindingLiquidSdk>>
abstract class BindingLiquidSdk implements RustOpaqueInterface {
  Stream<LiquidSdkEvent> addEventListener();

  void backup({required BackupRequest req});

  Future<void> disconnect();

  void emptyWalletCache();

  Future<GetInfoResponse> getInfo();

  Future<List<Payment>> listPayments();

  Future<PrepareReceiveResponse> prepareReceivePayment({required PrepareReceiveRequest req});

  Future<PrepareSendResponse> prepareSendPayment({required PrepareSendRequest req});

  Future<ReceivePaymentResponse> receivePayment({required PrepareReceiveResponse req});

  void restore({required RestoreRequest req});

  Future<SendPaymentResponse> sendPayment({required PrepareSendResponse req});

  Future<void> sync();
}

class BindingEventListener {
  final RustStreamSink<LiquidSdkEvent> stream;

  const BindingEventListener({
    required this.stream,
  });

  Future<void> onEvent({required LiquidSdkEvent e}) =>
      RustLib.instance.api.crateBindingsBindingEventListenerOnEvent(that: this, e: e);

  @override
  int get hashCode => stream.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is BindingEventListener && runtimeType == other.runtimeType && stream == other.stream;
}
