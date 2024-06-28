import 'dart:async';

import 'package:flutter/material.dart';
import 'package:flutter_breez_liquid_example/routes/home/widgets/balance.dart';
import 'package:flutter_breez_liquid_example/routes/home/widgets/bottom_app_bar.dart';
import 'package:flutter_breez_liquid_example/routes/home/widgets/drawer.dart';
import 'package:flutter_breez_liquid_example/routes/home/widgets/payment_list/payment_list.dart';
import 'package:flutter_breez_liquid_example/routes/home/widgets/qr_scan_action_button.dart';
import 'package:flutter_breez_liquid_example/services/breez_sdk_liquid.dart';
import 'package:flutter_breez_liquid_example/services/credentials_manager.dart';

class HomePage extends StatefulWidget {
  final CredentialsManager credentialsManager;
  final BreezSDKLiquid liquidSDK;

  const HomePage({super.key, required this.credentialsManager, required this.liquidSDK});

  @override
  State<HomePage> createState() => _HomePageState();
}

class _HomePageState extends State<HomePage> {
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(
          title: const Text('Breez Liquid SDK Demo'),
          titleTextStyle: const TextStyle(fontSize: 16.0, color: Colors.blue),
          backgroundColor: Colors.white,
          foregroundColor: Colors.blue,
          leading: Builder(
            builder: (context) {
              return IconButton(
                icon: const Icon(Icons.menu),
                onPressed: () {
                  Scaffold.of(context).openDrawer();
                },
              );
            },
          ),
          actions: const [],
        ),
        body: LayoutBuilder(
          builder: (BuildContext context, BoxConstraints constraints) {
            return Column(
              mainAxisSize: MainAxisSize.min,
              mainAxisAlignment: MainAxisAlignment.center,
              children: <Widget>[
                Container(
                  height: constraints.maxHeight * 0.3,
                  color: Colors.white,
                  child: Balance(walletInfoStream: widget.liquidSDK.walletInfoStream),
                ),
                Container(
                  height: constraints.maxHeight * 0.7,
                  color: Colors.white,
                  child: PaymentList(
                    paymentsStream: widget.liquidSDK.paymentsStream,
                    onRefresh: () async => await _sync(),
                  ),
                ),
              ],
            );
          },
        ),
        drawer: HomePageDrawer(
            liquidSDK: widget.liquidSDK.wallet!, credentialsManager: widget.credentialsManager),
        floatingActionButton: QrActionButton(liquidSDK: widget.liquidSDK.wallet!),
        floatingActionButtonLocation: FloatingActionButtonLocation.centerDocked,
        bottomNavigationBar: HomePageBottomAppBar(
          liquidSDK: widget.liquidSDK.wallet!,
          paymentsStream: widget.liquidSDK.paymentsStream,
        ),
      ),
    );
  }

  Future<void> _sync() async {
    try {
      debugPrint("Syncing wallet.");
      await widget.liquidSDK.wallet!.sync();
      debugPrint("Wallet synced!");
    } on Exception catch (e) {
      final errMsg = "Failed to sync wallet. $e";
      debugPrint(errMsg);
      if (context.mounted) {
        final snackBar = SnackBar(behavior: SnackBarBehavior.floating, content: Text(errMsg));
        ScaffoldMessenger.of(context).showSnackBar(snackBar);
      }
    }
  }
}
