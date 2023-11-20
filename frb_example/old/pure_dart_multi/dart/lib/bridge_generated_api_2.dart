// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`@ 1.82.4.
// ignore_for_file: non_constant_identifier_names, unused_element, duplicate_ignore, directives_ordering, curly_braces_in_flow_control_structures, unnecessary_lambdas, slash_for_doc_comments, prefer_const_literals_to_create_immutables, implicit_dynamic_list_literal, duplicate_import, unused_import, unnecessary_import, prefer_single_quotes, prefer_const_constructors, use_super_parameters, always_use_package_imports, annotate_overrides, invalid_use_of_protected_member, constant_identifier_names, invalid_use_of_internal_member, prefer_is_empty, unnecessary_const

import 'dart:convert';
import 'dart:async';
import 'package:meta/meta.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:uuid/uuid.dart';

import 'dart:convert';
import 'dart:async';
import 'package:meta/meta.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:uuid/uuid.dart';
import 'bridge_generated_api_2.io.dart' if (dart.library.html) 'bridge_generated_api_2.web.dart';

abstract class ApiClass2 {
  /// Documentation on a simple adder function.
  Future<int> simpleAdder2({required int a, required int b, dynamic hint});

  FlutterRustBridgeTaskConstMeta get kSimpleAdder2ConstMeta;
}

class ApiClass2Impl implements ApiClass2 {
  final ApiClass2Platform _platform;
  factory ApiClass2Impl(ExternalLibrary dylib) => ApiClass2Impl.raw(ApiClass2Platform(dylib));

  /// Only valid on web/WASM platforms.
  factory ApiClass2Impl.wasm(FutureOr<WasmModule> module) => ApiClass2Impl(module as ExternalLibrary);
  ApiClass2Impl.raw(this._platform);
  Future<int> simpleAdder2({required int a, required int b, dynamic hint}) {
    var arg0 = api2wire_i32(a);
    var arg1 = api2wire_i32(b);
    return _platform.executeNormal(FlutterRustBridgeTask(
      callFfi: (port_) => _platform.inner.wire_simple_adder_2(port_, arg0, arg1),
      parseSuccessData: _wire2api_i32,
      parseErrorData: null,
      constMeta: kSimpleAdder2ConstMeta,
      argValues: [a, b],
      hint: hint,
    ));
  }

  FlutterRustBridgeTaskConstMeta get kSimpleAdder2ConstMeta => const FlutterRustBridgeTaskConstMeta(
        debugName: "simple_adder_2",
        argNames: ["a", "b"],
      );

  void dispose() {
    _platform.dispose();
  }
// Section: wire2api

  int _wire2api_i32(dynamic raw) {
    return raw as int;
  }
}

// Section: api2wire

@protected
int api2wire_i32(int raw) {
  return raw;
}
// Section: finalizer