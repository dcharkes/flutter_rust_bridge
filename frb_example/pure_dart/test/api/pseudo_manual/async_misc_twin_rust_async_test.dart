// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `async_misc_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/async_misc_twin_rust_async.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('dart call funcAsyncVoid', () async {
    await funcAsyncVoid();
  });

  test('dart call funcAsyncSimpleAdd', () async {
    expect(await funcAsyncSimpleAdd(a: 10, b: 20), 30);
  });
}
