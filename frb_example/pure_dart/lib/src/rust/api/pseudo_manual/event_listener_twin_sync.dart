// ignore_for_file: invalid_use_of_internal_member, unused_import

import '../../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:freezed_annotation/freezed_annotation.dart' hide protected;
part 'event_listener_twin_sync.freezed.dart';

Stream<EventTwinSync> registerEventListenerTwinSync({dynamic hint}) =>
    RustLib.instance.api.registerEventListenerTwinSync(hint: hint);

void closeEventListenerTwinSync({dynamic hint}) =>
    RustLib.instance.api.closeEventListenerTwinSync(hint: hint);

void createEventTwinSync(
        {required String address, required String payload, dynamic hint}) =>
    RustLib.instance.api
        .createEventTwinSync(address: address, payload: payload, hint: hint);

@freezed
class EventTwinSync with _$EventTwinSync {
  const EventTwinSync._();
  const factory EventTwinSync({
    required String address,
    required String payload,
  }) = _EventTwinSync;
  String asStringTwinSync({dynamic hint}) =>
      RustLib.instance.api.eventTwinSyncAsStringTwinSync(
        that: this,
      );
}