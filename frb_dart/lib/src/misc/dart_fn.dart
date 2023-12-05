import 'package:flutter_rust_bridge/src/main_components/handler.dart';
import 'package:flutter_rust_bridge/src/utils/base_lazy_port_manager.dart';

/// {@macro flutter_rust_bridge.only_for_generated_code}
class DartFnInvokePortManager extends BaseLazyPortManager {
  final BaseHandler _handler;

  /// {@macro flutter_rust_bridge.internal}
  DartFnInvokePortManager(this._handler);

  @override
  void onData(Object? message) => _handler.dartFnInvoke(message);
}