import 'package:flutter/material.dart';
import 'package:flutter_breez_liquid/flutter_breez_liquid.dart';
import 'package:flutter_breez_liquid_example/routes/connect/connect_page.dart';
import 'package:flutter_breez_liquid_example/routes/home/home_page.dart';
import 'package:flutter_breez_liquid_example/services/credentials_manager.dart';
import 'package:flutter_breez_liquid_example/services/keychain.dart';
import 'package:flutter_breez_liquid_example/utils/config.dart';

import 'services/breez_liquid_sdk.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await initialize();
  final BreezLiquidSDK liquidSDK = BreezLiquidSDK();
  final credentialsManager = CredentialsManager(keyChain: KeyChain());
  final mnemonic = await credentialsManager.restoreMnemonic();
  if (mnemonic.isNotEmpty) {
    reconnect(liquidSDK: liquidSDK, mnemonic: mnemonic);
  }
  runApp(App(credentialsManager: credentialsManager, liquidSDK: liquidSDK));
}

Future<BindingLiquidSdk> reconnect({
  required BreezLiquidSDK liquidSDK,
  required String mnemonic,
  Network network = Network.mainnet,
}) async {
  final config = await getConfig(network: network);
  final req = ConnectRequest(
    config: config,
    mnemonic: mnemonic,
  );
  return await liquidSDK.connect(req: req);
}

class App extends StatefulWidget {
  final CredentialsManager credentialsManager;
  final BreezLiquidSDK liquidSDK;
  const App({super.key, required this.credentialsManager, required this.liquidSDK});

  static const title = 'Breez Liquid SDK Demo';

  @override
  State<App> createState() => _AppState();
}

class _AppState extends State<App> {
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: App.title,
      theme: ThemeData.from(colorScheme: ColorScheme.fromSeed(seedColor: Colors.white), useMaterial3: true),
      home: widget.liquidSDK.wallet == null
          ? ConnectPage(
              liquidSDK: widget.liquidSDK,
              credentialsManager: widget.credentialsManager,
            )
          : HomePage(
              credentialsManager: widget.credentialsManager,
              liquidSDK: widget.liquidSDK,
            ),
    );
  }
}
