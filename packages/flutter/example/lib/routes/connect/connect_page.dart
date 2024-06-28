import 'package:bip39/bip39.dart';
import 'package:flutter/material.dart';
import 'package:flutter_breez_liquid/flutter_breez_liquid.dart';
import 'package:flutter_breez_liquid_example/routes/connect/restore_page.dart';
import 'package:flutter_breez_liquid_example/routes/home/home_page.dart';
import 'package:flutter_breez_liquid_example/services/breez_sdk_liquid.dart';
import 'package:flutter_breez_liquid_example/services/credentials_manager.dart';
import 'package:flutter_breez_liquid_example/utils/config.dart';

class ConnectPage extends StatefulWidget {
  final BreezSDKLiquid liquidSDK;
  final CredentialsManager credentialsManager;
  const ConnectPage({super.key, required this.liquidSDK, required this.credentialsManager});

  @override
  State<ConnectPage> createState() => _ConnectPageState();
}

class _ConnectPageState extends State<ConnectPage> {
  bool connecting = false;

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(
          title: const Text('Breez Liquid SDK Demo'),
          foregroundColor: Colors.blue,
        ),
        body: Center(
          child: connecting
              ? const CircularProgressIndicator(color: Colors.blue)
              : Column(
                  mainAxisAlignment: MainAxisAlignment.center,
                  children: [
                    Padding(
                      padding: const EdgeInsets.symmetric(vertical: 16.0),
                      child: SizedBox(
                        width: 200,
                        child: ElevatedButton(
                          child: const Text("Create new wallet"),
                          onPressed: () async {
                            await createWallet();
                          },
                        ),
                      ),
                    ),
                    Padding(
                      padding: const EdgeInsets.symmetric(vertical: 16.0),
                      child: SizedBox(
                        width: 200,
                        child: ElevatedButton(
                          child: const Text("Restore from backup"),
                          onPressed: () {
                            Navigator.push(
                              context,
                              MaterialPageRoute(
                                builder: (BuildContext context) {
                                  return RestorePage(
                                    onRestore: (mnemonic) async {
                                      return await createWallet(mnemonic: mnemonic);
                                    },
                                  );
                                },
                              ),
                            );
                          },
                        ),
                      ),
                    )
                  ],
                ),
        ),
      ),
    );
  }

  Future<Null> createWallet({String? mnemonic}) async {
    final walletMnemonic = mnemonic ??= generateMnemonic(strength: 128);
    debugPrint("${mnemonic.isEmpty ? "Creating" : "Restoring"} wallet with $walletMnemonic");
    return await initializeWallet(mnemonic: walletMnemonic).then(
      (liquidSDK) async {
        await widget.credentialsManager.storeMnemonic(mnemonic: walletMnemonic).then((_) {
          Navigator.pushReplacement(
            context,
            MaterialPageRoute(
              builder: (BuildContext context) => HomePage(
                liquidSDK: widget.liquidSDK,
                credentialsManager: widget.credentialsManager,
              ),
            ),
          );
        });
      },
    );
  }

  Future<BindingLiquidSdk> initializeWallet({
    required String mnemonic,
    LiquidNetwork network = LiquidNetwork.mainnet,
  }) async {
    final config = await getConfig(network: network);
    final req = ConnectRequest(
      config: config,
      mnemonic: mnemonic,
    );
    return await widget.liquidSDK.connect(req: req);
  }
}
