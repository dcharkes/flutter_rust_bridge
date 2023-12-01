// ignore_for_file: invalid_use_of_internal_member, unused_import

import '../../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

Future<void> benchmarkVoidTwinRustAsync({dynamic hint}) =>
    RustLib.instance.api.benchmarkVoidTwinRustAsync(hint: hint);

Future<int> benchmarkInputBytesTwinRustAsync(
        {required Uint8List bytes, dynamic hint}) =>
    RustLib.instance.api
        .benchmarkInputBytesTwinRustAsync(bytes: bytes, hint: hint);

Future<Uint8List> benchmarkOutputBytesTwinRustAsync(
        {required int size, dynamic hint}) =>
    RustLib.instance.api
        .benchmarkOutputBytesTwinRustAsync(size: size, hint: hint);