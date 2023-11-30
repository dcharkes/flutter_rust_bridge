// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `optional_primitive_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/optional_primitive_twin_rust_async.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';
import '../../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  group('optional_primitive', () {
    addTestsIdentityFunctionCall(exampleOptionalPrimitiveTypeI8TwinRustAsync,
        <int?>[null, 0, -128, 127]);
    addTestsIdentityFunctionCall(exampleOptionalPrimitiveTypeI16TwinRustAsync,
        <int?>[null, 0, -32768, 32767]);
    addTestsIdentityFunctionCall(exampleOptionalPrimitiveTypeI32TwinRustAsync,
        <int?>[null, 0, -2147483648, 2147483647]);
    addTestsIdentityFunctionCall(exampleOptionalPrimitiveTypeI64TwinRustAsync,
        <int?>[null, 0, -9007199254740992, 9007199254740992]);
    addTestsIdentityFunctionCall(
        exampleOptionalPrimitiveTypeU8TwinRustAsync, <int?>[null, 0, 255]);
    addTestsIdentityFunctionCall(
        exampleOptionalPrimitiveTypeU16TwinRustAsync, <int?>[null, 0, 65535]);
    addTestsIdentityFunctionCall(exampleOptionalPrimitiveTypeU32TwinRustAsync,
        <int?>[null, 0, 4294967295]);
    addTestsIdentityFunctionCall(exampleOptionalPrimitiveTypeU64TwinRustAsync,
        <int?>[null, 0, 9007199254740992]);
    addTestsIdentityFunctionCall(exampleOptionalPrimitiveTypeF32TwinRustAsync,
        <double?>[null, 0, -42.5, 123456]);
    addTestsIdentityFunctionCall(exampleOptionalPrimitiveTypeF64TwinRustAsync,
        <double?>[null, 0, -42.5, 123456]);
    addTestsIdentityFunctionCall(exampleOptionalPrimitiveTypeBoolTwinRustAsync,
        <bool?>[null, false, true]);
  });
}
