#!/usr/bin/env dart

import 'dart:io';

import 'package:args/args.dart';
import 'package:cli_script/cli_script.dart';

import 'utils.dart';

const libName = 'breez_sdk_liquid';
const linuxLibName = 'lib$libName.so';
const windowsLibName = '$libName.dll';
const buildDir = 'platform-build';

Future<void> mainImpl(List<String> args) async {
  final parser = ArgParser()
    ..addFlag('debug')
    ..addFlag('local')
    ..addOption('profile');
  final opts = parser.parse(args);

  String profile, profileArg;
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

  // -- Begin --
  await run('mkdir -p $buildDir');
  Directory.current = buildDir;

  await run('cargo install cargo-zigbuild cargo-xwin');

  final targets = opts['local'] ? [Targets.host] : Targets.values;
  final compilerOpts = opts.rest;
  for (final target in targets) {
    final triple = target.triple;
    final flutterIdentifier = target.flutterIdentifier;
    await run('rustup target add $triple');
    await run('${target.compiler} --package breez-liquid-sdk --target $triple $profileArg',
        args: compilerOpts);
    await run('mkdir -p $flutterIdentifier');
    await run('cp ../../../../target/$triple/$profile/${target.libName} $flutterIdentifier/');
  }

  final hasLinux = targets.any((target) => !target.isWindows);
  final hasWindows = targets.any((target) => target.isWindows);
  await run('tar -czvf other.tar.gz', args: [
    if (hasLinux) ...'linux-*'.glob,
    if (hasWindows) ...'windows-*'.glob,
  ]);
}

void main(List<String> args) {
  wrapMain(() async {
    try {
      await mainImpl(args);
    } finally {
      await check('rm -rf linux-* windows-*');
    }
  });
}

enum Targets {
  linuxArm64('aarch64-unknown-linux-gnu', 'linux-arm64'),
  linuxX64('x86_64-unknown-linux-gnu', 'linux-x64');
  // TODO: Enable builds for Windows targets
  //windowsArm64('aarch64-pc-windows-msvc', 'windows-arm64', isWindows: true),
  //windowsX64('x86_64-pc-windows-msvc', 'windows-x64', isWindows: true);

  final String triple;
  final String flutterIdentifier;
  final bool isWindows;
  // ignore: unused_element
  const Targets(this.triple, this.flutterIdentifier, {this.isWindows = false});

  static Targets get host {
    final host = hostTarget;
    return values.firstWhere((target) => target.triple == host);
  }

  String get compiler =>
      isWindows ? 'cargo xwin build --package breez-liquid-sdk' : 'cargo zigbuild --package breez-liquid-sdk';
  String get libName => isWindows ? windowsLibName : linuxLibName;
}
