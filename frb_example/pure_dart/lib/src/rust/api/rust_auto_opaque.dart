// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 1.82.4.

// ignore_for_file: invalid_use_of_internal_member, unused_import

import '../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

Future<void> rustAutoOpaqueArgOwn(
        {required NonCloneSimpleTwinNormal arg, dynamic hint}) =>
    RustLib.instance.api.rustAutoOpaqueArgOwn(arg: arg, hint: hint);

Future<void> rustAutoOpaqueArgBorrow(
        {required NonCloneSimpleTwinNormal arg, dynamic hint}) =>
    RustLib.instance.api.rustAutoOpaqueArgBorrow(arg: arg, hint: hint);

Future<void> rustAutoOpaqueArgMutBorrow(
        {required NonCloneSimpleTwinNormal arg, dynamic hint}) =>
    RustLib.instance.api.rustAutoOpaqueArgMutBorrow(arg: arg, hint: hint);

Future<NonCloneSimpleTwinNormal> rustAutoOpaqueReturnOwn({dynamic hint}) =>
    RustLib.instance.api.rustAutoOpaqueReturnOwn(hint: hint);

Future<NonCloneSimpleTwinNormal> rustAutoOpaqueArgOwnAndReturnOwn(
        {required NonCloneSimpleTwinNormal arg, dynamic hint}) =>
    RustLib.instance.api.rustAutoOpaqueArgOwnAndReturnOwn(arg: arg, hint: hint);

Future<void> rustAutoOpaqueTwoArgs(
        {required NonCloneSimpleTwinNormal a,
        required NonCloneSimpleTwinNormal b,
        dynamic hint}) =>
    RustLib.instance.api.rustAutoOpaqueTwoArgs(a: a, b: b, hint: hint);

Future<void> rustAutoOpaqueNormalAndOpaqueArg(
        {required NonCloneSimpleTwinNormal a,
        required String b,
        dynamic hint}) =>
    RustLib.instance.api
        .rustAutoOpaqueNormalAndOpaqueArg(a: a, b: b, hint: hint);

/// "+" inside the type signature
Future<void> rustAutoOpaquePlusSignArg(
        {required BoxMyTraitTwinNormal arg, dynamic hint}) =>
    RustLib.instance.api.rustAutoOpaquePlusSignArg(arg: arg, hint: hint);

Future<BoxMyTraitTwinNormal> rustAutoOpaquePlusSignReturn({dynamic hint}) =>
    RustLib.instance.api.rustAutoOpaquePlusSignReturn(hint: hint);

Future<void> rustAutoOpaqueCallableArg(
        {required BoxFnStringString arg, dynamic hint}) =>
    RustLib.instance.api.rustAutoOpaqueCallableArg(arg: arg, hint: hint);

Future<BoxFnStringString> rustAutoOpaqueCallableReturn({dynamic hint}) =>
    RustLib.instance.api.rustAutoOpaqueCallableReturn(hint: hint);

Future<void> rustAutoOpaqueTraitObjectArgOwn(
        {required BoxHelloTraitTwinNormal arg,
        required String expect,
        dynamic hint}) =>
    RustLib.instance.api
        .rustAutoOpaqueTraitObjectArgOwn(arg: arg, expect: expect, hint: hint);

Future<void> rustAutoOpaqueTraitObjectArgBorrow(
        {required BoxHelloTraitTwinNormal arg,
        required String expect,
        dynamic hint}) =>
    RustLib.instance.api.rustAutoOpaqueTraitObjectArgBorrow(
        arg: arg, expect: expect, hint: hint);

Future<void> rustAutoOpaqueTraitObjectArgMutBorrow(
        {required BoxHelloTraitTwinNormal arg,
        required String expect,
        dynamic hint}) =>
    RustLib.instance.api.rustAutoOpaqueTraitObjectArgMutBorrow(
        arg: arg, expect: expect, hint: hint);

Future<BoxHelloTraitTwinNormal> rustAutoOpaqueTraitObjectReturnOwnOne(
        {dynamic hint}) =>
    RustLib.instance.api.rustAutoOpaqueTraitObjectReturnOwnOne(hint: hint);

Future<BoxHelloTraitTwinNormal> rustAutoOpaqueTraitObjectReturnOwnTwo(
        {dynamic hint}) =>
    RustLib.instance.api.rustAutoOpaqueTraitObjectReturnOwnTwo(hint: hint);

Future<void> rustAutoOpaqueStructWithGoodAndOpaqueFieldArgOwn(
        {required StructWithGoodAndOpaqueFieldTwinNormal arg, dynamic hint}) =>
    RustLib.instance.api
        .rustAutoOpaqueStructWithGoodAndOpaqueFieldArgOwn(arg: arg, hint: hint);

Future<void> rustAutoOpaqueStructWithGoodAndOpaqueFieldArgBorrow(
        {required StructWithGoodAndOpaqueFieldTwinNormal arg, dynamic hint}) =>
    RustLib.instance.api.rustAutoOpaqueStructWithGoodAndOpaqueFieldArgBorrow(
        arg: arg, hint: hint);

Future<void> rustAutoOpaqueStructWithGoodAndOpaqueFieldArgMutBorrow(
        {required StructWithGoodAndOpaqueFieldTwinNormal arg, dynamic hint}) =>
    RustLib.instance.api.rustAutoOpaqueStructWithGoodAndOpaqueFieldArgMutBorrow(
        arg: arg, hint: hint);

Future<StructWithGoodAndOpaqueFieldTwinNormal>
    rustAutoOpaqueStructWithGoodAndOpaqueFieldReturnOwn({dynamic hint}) =>
        RustLib.instance.api
            .rustAutoOpaqueStructWithGoodAndOpaqueFieldReturnOwn(hint: hint);

// Rust type: Box < dyn HelloTraitTwinNormal >
@sealed
class BoxHelloTraitTwinNormal extends RustAutoOpaque {
  BoxHelloTraitTwinNormal.fromWire(dynamic wire)
      : super.fromWire(wire, _kStaticData);

  static final _kStaticData = RustArcStaticData(
    rustArcIncrementStrongCount: RustLib
        .instance.api.rust_arc_increment_strong_count_BoxHelloTraitTwinNormal,
    rustArcDecrementStrongCount: RustLib
        .instance.api.rust_arc_decrement_strong_count_BoxHelloTraitTwinNormal,
    rustArcDecrementStrongCountPtr: RustLib.instance.api
        .rust_arc_decrement_strong_count_BoxHelloTraitTwinNormalPtr,
  );
}

// Rust type: NonCloneSimpleTwinNormal
@sealed
class NonCloneSimpleTwinNormal extends RustAutoOpaque {
  NonCloneSimpleTwinNormal.fromWire(dynamic wire)
      : super.fromWire(wire, _kStaticData);

  static final _kStaticData = RustArcStaticData(
    rustArcIncrementStrongCount: RustLib
        .instance.api.rust_arc_increment_strong_count_NonCloneSimpleTwinNormal,
    rustArcDecrementStrongCount: RustLib
        .instance.api.rust_arc_decrement_strong_count_NonCloneSimpleTwinNormal,
    rustArcDecrementStrongCountPtr: RustLib.instance.api
        .rust_arc_decrement_strong_count_NonCloneSimpleTwinNormalPtr,
  );
}

// Rust type: StructWithGoodAndOpaqueFieldTwinNormal
@sealed
class StructWithGoodAndOpaqueFieldTwinNormal extends RustAutoOpaque {
  StructWithGoodAndOpaqueFieldTwinNormal.fromWire(dynamic wire)
      : super.fromWire(wire, _kStaticData);

  static final _kStaticData = RustArcStaticData(
    rustArcIncrementStrongCount: RustLib.instance.api
        .rust_arc_increment_strong_count_StructWithGoodAndOpaqueFieldTwinNormal,
    rustArcDecrementStrongCount: RustLib.instance.api
        .rust_arc_decrement_strong_count_StructWithGoodAndOpaqueFieldTwinNormal,
    rustArcDecrementStrongCountPtr: RustLib.instance.api
        .rust_arc_decrement_strong_count_StructWithGoodAndOpaqueFieldTwinNormalPtr,
  );
}

// Rust type: Box<dyn Fn (String) -> String>
@sealed
class BoxFnStringString extends RustAutoOpaque {
  BoxFnStringString.fromWire(dynamic wire) : super.fromWire(wire, _kStaticData);

  static final _kStaticData = RustArcStaticData(
    rustArcIncrementStrongCount:
        RustLib.instance.api.rust_arc_increment_strong_count_BoxFnStringString,
    rustArcDecrementStrongCount:
        RustLib.instance.api.rust_arc_decrement_strong_count_BoxFnStringString,
    rustArcDecrementStrongCountPtr: RustLib
        .instance.api.rust_arc_decrement_strong_count_BoxFnStringStringPtr,
  );
}

// Rust type: Box<dyn HelloTraitTwinNormal>
@sealed
class BoxHelloTraitTwinNormal extends RustAutoOpaque {
  BoxHelloTraitTwinNormal.fromWire(dynamic wire)
      : super.fromWire(wire, _kStaticData);

  static final _kStaticData = RustArcStaticData(
    rustArcIncrementStrongCount: RustLib
        .instance.api.rust_arc_increment_strong_count_BoxHelloTraitTwinNormal,
    rustArcDecrementStrongCount: RustLib
        .instance.api.rust_arc_decrement_strong_count_BoxHelloTraitTwinNormal,
    rustArcDecrementStrongCountPtr: RustLib.instance.api
        .rust_arc_decrement_strong_count_BoxHelloTraitTwinNormalPtr,
  );
}

// Rust type: Box<dyn MyTraitTwinNormal + Send + Sync>
@sealed
class BoxMyTraitTwinNormal extends RustAutoOpaque {
  BoxMyTraitTwinNormal.fromWire(dynamic wire)
      : super.fromWire(wire, _kStaticData);

  static final _kStaticData = RustArcStaticData(
    rustArcIncrementStrongCount: RustLib
        .instance.api.rust_arc_increment_strong_count_BoxMyTraitTwinNormal,
    rustArcDecrementStrongCount: RustLib
        .instance.api.rust_arc_decrement_strong_count_BoxMyTraitTwinNormal,
    rustArcDecrementStrongCountPtr: RustLib
        .instance.api.rust_arc_decrement_strong_count_BoxMyTraitTwinNormalPtr,
  );
}