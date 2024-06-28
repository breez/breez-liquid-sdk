import 'dart:io';

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:breez_liquid/breez_liquid.dart';
import 'package:path/path.dart' as p;

final hostTriple = () {
  final result = Process.runSync('rustc', const ['-vV']);

  return (result.stdout as String)
      .split('\n')
      .firstWhere((line) => line.startsWith('host:'))
      .split(':')
      .last
      .trim();
}();

extension on String {
  String get dylib {
    if (Platform.isWindows) {
      return '$this.dll';
    }
    if (Platform.isMacOS) {
      return 'lib$this.dylib';
    }
    return 'lib$this.so';
  }
}

String dylibPath(String profile) => Uri.base
    .resolve(p.joinAll([
      '../../lib/target',
      if (Platform.isMacOS && hostTriple.startsWith('aarch64')) hostTriple,
      profile,
      'breez_sdk_liquid'.dylib,
    ]))
    .toFilePath();

Future<void> initApi({String profile = 'frb-min'}) {
  return initialize(dylib: ExternalLibrary.open(dylibPath(profile)));
}
