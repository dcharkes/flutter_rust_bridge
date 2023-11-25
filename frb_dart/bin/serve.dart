// ignore_for_file: avoid_print

import 'dart:convert';
import 'dart:io';

import 'package:build_cli_annotations/build_cli_annotations.dart';
import 'package:path/path.dart' as p;
import 'package:puppeteer/puppeteer.dart';
import 'package:shelf/shelf.dart';
import 'package:shelf/shelf_io.dart';
import 'package:shelf_static/shelf_static.dart';
import 'package:shelf_web_socket/shelf_web_socket.dart';
import 'package:web_socket_channel/web_socket_channel.dart';
import 'package:yaml/yaml.dart';

final YamlMap? pubspec = () {
  final pubspecPath = Platform.script.resolve('../pubspec.yaml');
  final pubpsec = File(pubspecPath.toFilePath());
  try {
    return loadYaml(pubpsec.readAsStringSync(), sourceUrl: pubspecPath);
  } catch (err) {
    eprint('Failed to read pubspec: $err');
  }
}();

String get version => pubspec?['version'] ?? '';

final which = Platform.isWindows ? 'where.exe' : 'which';

String err(String msg) {
  // return stderr.supportsAnsiEscapes ? Colorize(msg).red().bold().toString() : msg; // #1262
  return msg;
}

void eprint([Object? msg = 'unspecified']) {
  stderr.writeln('${err('error')}: $msg');
}

Never bail([String? message]) {
  eprint(message);
  exit(1);
}

void main(List<String> args) async {
  const exec = 'flutter_rust_bridge_serve';
  final config = parseOpts(args);
  if (config.help) {
    print("""
$exec $version
Develop Rust WASM modules with cross-origin isolation.

USAGE:
\t$exec [OPTIONS] [..REST]
\t$exec --dart-input <ENTRY> --root <ROOT> [OPTIONS] [..REST]

OPTIONS:""");
    print(_$parserForOpts.usage);
    return;
  }

  await runCommand(which, ['wasm-pack']).catchError((_) {
    bail(
      'wasm-pack is required, but not found in the path.\n'
      'Please install wasm-pack by following the instructions at https://rustwasm.github.io/wasm-pack/\n'
      'or running `cargo install wasm-pack`.',
    );
  });

  if (config.shouldRunBindgen) {
    await runCommand(which, ['wasm-bindgen']).catchError((_) {
      bail(
        'wasm-bindgen flags are enabled, but wasm-bindgen could not be found in the path.\n'
        'Please install wasm-bindgen using `cargo install -f wasm-bindgen-cli`.',
      );
    });
  }

  final String root;
  final String wasmOutput;
  if (config.dartInput != null) {
    if (config.root == null) {
      bail('The --root option is required when building plain Dart projects.');
    }
    root = p.canonicalize(config.root!);
    wasmOutput = p.canonicalize(config.wasmOutput ?? '$root/pkg');
  } else {
    root = p.canonicalize(config.root ?? 'build/web');
    wasmOutput = p.canonicalize(config.wasmOutput ?? 'web/pkg');
  }

  final crateDir = config.crate;
  if (!await File('$crateDir/Cargo.toml').exists()) {
    bail(
      '$crateDir is not a crate directory.\n'
      'Please specify the crate directory using "--crate <CRATE>".',
    );
  }

  // --- Checks end ---

  if (config.build) {
    await build(
      config,
      crateDir: crateDir,
      wasmOutput: wasmOutput,
      root: root,
      args: args,
    );
  }
  await runServer(config, root: root);
}

Future<void> build(
  Opts config, {
  required String crateDir,
  required String wasmOutput,
  required String root,
  required List<String> args,
}) async {
  final manifest = jsonDecode(await runCommand(
    'cargo',
    ['read-manifest'],
    pwd: crateDir,
    silent: true,
  ));
  final String crateName =
      (manifest['targets'] as List).firstWhere((target) => (target['kind'] as List).contains('cdylib'))['name'];
  if (crateName.isEmpty) bail('Crate name cannot be empty.');
  await runCommand('wasm-pack', [
    'build', '-t', 'no-modules', '-d', wasmOutput, '--no-typescript',
    '--out-name', crateName,
    if (!config.release) '--dev', crateDir,
    '--', // cargo build args
    '-Z', 'build-std=std,panic_abort',
    if (config.noDefaultFeatures) '--no-default-features',
    if (config.features != null) '--features=${config.features}'
  ], env: {
    'RUSTUP_TOOLCHAIN': 'nightly',
    'RUSTFLAGS': '-C target-feature=+atomics,+bulk-memory,+mutable-globals',
    if (stdout.supportsAnsiEscapes) 'CARGO_TERM_COLOR': 'always',
  });
  if (config.shouldRunBindgen) {
    await runCommand('wasm-bindgen', [
      '$crateDir/target/wasm32-unknown-unknown/${config.release ? 'release' : 'debug'}/$crateName.wasm',
      '--out-dir',
      wasmOutput,
      '--no-typescript',
      '--target',
      'no-modules',
      if (config.weakRefs) '--weak-refs',
      if (config.referenceTypes) '--reference-types',
    ]);
  }
  if (config.dartInput != null) {
    final output = p.basename(config.dartInput!);
    await runCommand('dart', [
      'compile',
      'js',
      '-o',
      '$root/$output.js',
      if (config.release) '-O2',
      if (stdout.supportsAnsiEscapes) '--enable-diagnostic-colors',
      if (config.verbose) '--verbose',
      config.dartInput!,
    ]);
  } else {
    await runCommand(
      'flutter',
      ['build', 'web', if (!config.release) '--profile'] + Opts.rest(args),
    );
  }
}
