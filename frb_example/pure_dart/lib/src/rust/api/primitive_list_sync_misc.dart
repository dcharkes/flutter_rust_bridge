// ignore_for_file: invalid_use_of_internal_member, unused_import

import '../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'primitive_list_misc.dart';

ZeroCopyVecOfPrimitivePack handleZeroCopyVecOfPrimitiveSync(
        {required int n, dynamic hint}) =>
    RustLib.instance.api.handleZeroCopyVecOfPrimitiveSync(n: n, hint: hint);