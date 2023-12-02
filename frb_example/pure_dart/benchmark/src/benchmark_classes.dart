// ignore_for_file: invalid_use_of_internal_member, invalid_use_of_protected_member

import 'dart:async';
import 'dart:ffi';
import 'dart:isolate';
import 'dart:math';
import 'dart:typed_data';

import 'package:benchmark_harness/benchmark_harness.dart';
import 'package:frb_example_pure_dart/src/rust/api/benchmark_api.dart';
import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/benchmark_api_twin_sync.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.io.dart';

import 'benchmark_utils.dart';

List<MaybeAsyncBenchmarkBase> createBenchmarks(
    {required ScoreEmitter emitter}) {
  return [
    // compute prime
    ComputePrimeBenchmark(90000049, emitter: emitter),
    ComputePrimeBenchmark(9000000001, emitter: emitter),
    ComputePrimeBenchmark(900000000013, emitter: emitter),

    // void
    VoidAsyncBenchmark(emitter: emitter),
    VoidSyncBenchmark(emitter: emitter),
    VoidSyncRawBenchmark(emitter: emitter),

    //
    for (final len in [0, 10000, 1000000]) ...[
      // input bytes
      InputBytesAsyncBenchmark(len, emitter: emitter),
      InputBytesSyncBenchmark(len, emitter: emitter),
      InputBytesSyncRawBenchmark(len, emitter: emitter),

      // output bytes
      OutputBytesAsyncBenchmark(len, emitter: emitter),
      OutputBytesSyncBenchmark(len, emitter: emitter),
      OutputBytesAsyncRawBenchmark(len, emitter: emitter),
    ],
  ];
}

late final RustLibWire _wire = (RustLib.instance.api as RustLibApiImpl).wire;

// For a list of primes: http://compoasso.free.fr/primelistweb/page/prime/liste_online_en.php
class ComputePrimeBenchmark extends EnhancedBenchmarkBase {
  final int number;

  const ComputePrimeBenchmark(this.number, {super.emitter})
      : super('ComputePrime_Number$number');

  @override
  void run() {
    final ans = isPrime(number);
    if (!ans) throw Exception('unexpected');
  }

  bool isPrime(int n) {
    final sqrtN = sqrt(n);
    for (var i = 2; i <= sqrtN; ++i) {
      if (n % i == 0) return false;
    }
    return true;
  }
}

class VoidAsyncBenchmark extends EnhancedAsyncBenchmarkBase {
  const VoidAsyncBenchmark({super.emitter}) : super('VoidAsync');

  @override
  Future<void> run() async => benchmarkVoidTwinNormal();
}

class VoidSyncBenchmark extends EnhancedBenchmarkBase {
  const VoidSyncBenchmark({super.emitter}) : super('VoidSync');

  @override
  void run() => benchmarkVoidTwinSync();
}

class VoidSyncRawBenchmark extends EnhancedBenchmarkBase {
  VoidSyncRawBenchmark({super.emitter}) : super('VoidSyncRaw');

  @override
  void run() => _wire.benchmark_raw_void_sync();
}

class InputBytesAsyncBenchmark extends EnhancedAsyncBenchmarkBase {
  final Uint8List bytes;

  InputBytesAsyncBenchmark(int len, {super.emitter})
      : bytes = Uint8List(len),
        super('InputBytesAsync_Len$len');

  @override
  Future<void> run() async => benchmarkInputBytesTwinNormal(bytes: bytes);
}

class InputBytesSyncBenchmark extends EnhancedBenchmarkBase {
  final Uint8List bytes;

  InputBytesSyncBenchmark(int len, {super.emitter})
      : bytes = Uint8List(len),
        super('InputBytesSync_Len$len');

  @override
  void run() => benchmarkInputBytesTwinSync(bytes: bytes);
}

class InputBytesSyncRawBenchmark extends EnhancedBenchmarkBase {
  final Uint8List bytes;

  InputBytesSyncRawBenchmark(int len, {super.emitter})
      : bytes = Uint8List(len),
        super('InputBytesSyncRaw_Len$len');

  @override
  void run() {
    final raw = _wire.benchmark_raw_new_list_prim_u_8(bytes.length);
    raw.ptr.asTypedList(raw.len).setAll(0, bytes);
    final ans = _wire.benchmark_raw_input_bytes(raw);
    if (ans != 0) throw Exception();
  }
}

class OutputBytesAsyncBenchmark extends EnhancedAsyncBenchmarkBase {
  final int len;

  OutputBytesAsyncBenchmark(this.len, {super.emitter})
      : super('OutputBytesAsync_Len$len');

  @override
  Future<void> run() async => benchmarkOutputBytesTwinNormal(size: len);
}

class OutputBytesSyncBenchmark extends EnhancedBenchmarkBase {
  final int len;

  OutputBytesSyncBenchmark(this.len, {super.emitter})
      : super('OutputBytesSync_Len$len');

  @override
  void run() => benchmarkOutputBytesTwinSync(size: len);
}

class OutputBytesAsyncRawBenchmark extends EnhancedAsyncBenchmarkBase {
  final receivePort = RawReceivePort();
  late final sendPort = receivePort.sendPort.nativePort;
  final int len;
  final completers = <int, Completer<Uint8List>>{};
  var nextId = 1;

  OutputBytesAsyncRawBenchmark(this.len, {super.emitter})
      : super('OutputBytesAsyncRaw_Len$len') {
    receivePort.handler = (dynamic response) {
      final bytes = response as Uint8List;
      final messageId = ByteData.view(bytes.buffer).getInt32(0, Endian.big);
      // indeed a sublist view of the bytes
      completers.remove(messageId)!.complete(bytes);
    };
  }

  @override
  Future<void> teardown() async {
    receivePort.close();
  }

  @override
  Future<void> run() async {
    final messageId = nextId++;
    final completer = Completer<Uint8List>();
    completers[messageId] = completer;

    _wire.benchmark_raw_output_bytes(sendPort, messageId, len);
    final result = await completer.future;

    // sanity check
    if (result.length != len + 4) throw Exception();
  }
}