// ignore_for_file: invalid_use_of_internal_member, unused_import

import '../../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

class NewSimpleStruct {
  final int field;

  const NewSimpleStruct({
    required this.field,
  });

  @override
  int get hashCode => field.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is NewSimpleStruct &&
          runtimeType == other.runtimeType &&
          field == other.field;
}