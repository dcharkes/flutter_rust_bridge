import 'dart:ffi' as ffi;

import 'package:ffi/ffi.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:frb_example_deliberate_bad/src/rust/api/simple.dart';
import 'package:frb_example_deliberate_bad/src/rust/frb_generated.dart';

Future<void> main(List<String> args) async {
  await RustLib.init(
    // because we unconventionally specified the `--target` in cargo build
    externalLibrary: ExternalLibrary.open(
        'rust/target/x86_64-unknown-linux-gnu/release/libfrb_example_deliberate_bad.so'),
  );

  switch (args[0]) {
    case 'DartOnly_Good':
      print('I am good Dart code');

    case 'DartOnly_HeapUseAfterFree':
      // ref https://github.com/dart-lang/sdk/blob/main/tests/ffi/calloc_test.dart
      // https://github.com/dart-lang/samples/blob/main/ffi/structs/structs.dart
      final p = calloc<ffi.Float>();
      print('read p firstly: ${p.value}');
      calloc.free(p);
      print('read p after free: ${p.value}');

    case 'DartCallRust_StackBufferOverflow':
      await makeStackBufferOverflow();

    case 'DartCallRust_HeapUseAfterFree':
      await makeHeapUseAfterFree();

    default:
      throw Exception('Unknown args $args');
  }
}