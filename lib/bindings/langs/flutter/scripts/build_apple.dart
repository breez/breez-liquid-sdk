#!/usr/bin/env dart

import 'dart:io';

import 'package:args/args.dart';
import 'package:cli_script/cli_script.dart';

import 'utils.dart';

const framework = 'breez_sdk_liquid.xcframework';
const frameworkZip = '$framework.zip';
const libName = 'libbreez_sdk_liquid.a';
const iosSimLipo = 'ios-sim-lipo/$libName';
const macLipo = 'mac-lipo/$libName';
const headers = '../breez_sdk_liquid/include';
const buildDir = 'platform-build';

Future<void> mainImpl(List<String> args) async {
  final parser = ArgParser()
    ..addFlag('debug', negatable: false)
    ..addFlag('local')
    ..addFlag('ios')
    ..addOption('profile');
  final opts = parser.parse(args);
  final observer = Observer();

  final String profile, profileArg;
  if (opts.wasParsed('profile')) {
    profile = opts['profile'];
    profileArg = '--profile=$profile';
  } else if (opts['debug']) {
    profile = 'debug';
    profileArg = '--profile=dev';
  } else if (opts['local']) {
    profile = 'frb';
    profileArg = '--profile=frb';
  } else {
    profile = 'frb-min';
    profileArg = '--profile=frb-min';
  }

  print(' Building profile: $profile');

  final List<String> targets;
  if (opts['local']) {
    targets = [hostTarget];
  } else if (opts['ios']) {
    targets = const [
      'aarch64-apple-ios',
      'x86_64-apple-ios',
      'aarch64-apple-ios-sim',
    ];
  } else {
    targets = const [
      'aarch64-apple-ios',
      'x86_64-apple-ios',
      'aarch64-apple-ios-sim',
      'x86_64-apple-darwin',
      'aarch64-apple-darwin',
    ];
  }

  print('for targets:\n- ${targets.join('\n- ')}');

  // -- Begin --

  await run('mkdir -p $buildDir');
  Directory.current = buildDir;

  final outputs = targets.map((target) {
    return observer.mark('../../../../target/$target/$profile/$libName');
  }).toList();

  for (final target in targets) {
    print(' Building target $target');
    await run('rustup target add $target');
    await run('cargo build --package breez-liquid-sdk --target=$target $profileArg');
  }

  await run('mkdir -p mac-lipo ios-sim-lipo');
  if (opts['local']) {
    final output = outputs.single;
    final isIos = output.contains('ios');
    final shouldBuildFramework = observer.hasChanged(output) || !fileExists(frameworkZip);

    String lipoOut;
    if (shouldBuildFramework) {
      lipoOut = isIos ? iosSimLipo : macLipo;
    } else {
      print('Nothing changed, exiting...');
      return;
    }

    await run('lipo -create -output $lipoOut $output');
    await run('xcodebuild -create-xcframework '
        '-library $lipoOut -headers $headers '
        '-output $framework');
  } else {
    final armIos = '../../../../target/aarch64-apple-ios/$profile/$libName';
    var shouldBuildFramework = !fileExists(frameworkZip) || observer.hasChanged(armIos);
    if (!fileExists(iosSimLipo) ||
        outputs.where((output) => output.contains('ios')).any(observer.hasChanged)) {
      shouldBuildFramework = true;
      await run('lipo -create -output $iosSimLipo '
          '../../../../target/aarch64-apple-ios-sim/$profile/$libName '
          '../../../../target/x86_64-apple-ios/$profile/$libName ');
    }
    if (!fileExists(macLipo) ||
        outputs.where((output) => output.contains('darwin')).any(observer.hasChanged)) {
      shouldBuildFramework = true;
      await run('lipo -create -output $macLipo '
          '../../../../target/aarch64-apple-darwin/$profile/$libName '
          '../../../../target/x86_64-apple-darwin/$profile/$libName');
    }
    if (shouldBuildFramework) {
      await run('xcodebuild -create-xcframework '
          '-library $iosSimLipo -headers $headers '
          '-library $macLipo -headers $headers '
          '-library $armIos -headers $headers '
          '-output $framework');
    }
  }

  print(' Creating $frameworkZip');
  await run('zip -ry $frameworkZip $framework');

  print('✅ Done!');
}

void main(List<String> args) {
  wrapMain(() async {
    try {
      await mainImpl(args);
    } finally {
      await check('rm -rf ios-sim-lipo mac-lipo $framework');
    }
  });
}
