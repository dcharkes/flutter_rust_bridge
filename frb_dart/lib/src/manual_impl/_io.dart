import 'dart:ffi';

import 'package:flutter_rust_bridge/src/platform_types/_io.dart';
import 'package:flutter_rust_bridge/src/utils/dart_c_object_into_dart.dart';
import 'package:meta/meta.dart';

/// Generates the dynamic Dart object from either an FFI struct or a JS value
///
/// {@macro flutter_rust_bridge.internal}
@internal
List<dynamic> wireSyncReturnIntoDart(WireSyncReturn syncReturn) => dartCObjectIntoDart(syncReturn.ref);