import 'package:benchmark_harness/benchmark_harness.dart';

abstract class MaybeAsyncBenchmarkBase {
  String get name;

  Future<void> reportMaybeAsync();

  Future<void> loop(int count);
}

abstract class EnhancedBenchmarkBase extends BenchmarkBase
    implements MaybeAsyncBenchmarkBase {
  const EnhancedBenchmarkBase(super.name, {super.emitter});

  Future<void> loop(int count) async {
    for (var i = 0; i < count; ++i) run();
  }

  @override
  Future<void> reportMaybeAsync() async => report();

  // To opt into the reporting the time per run() instead of per 10 run() calls.
  @override
  void exercise() => run();
}

abstract class EnhancedAsyncBenchmarkBase extends AsyncBenchmarkBase
    implements MaybeAsyncBenchmarkBase {
  const EnhancedAsyncBenchmarkBase(super.name, {super.emitter});

  Future<void> loop(int count) async {
    for (var i = 0; i < count; ++i) await run();
  }

  @override
  Future<void> reportMaybeAsync() async => report();
}

class JsonEmitter extends ScoreEmitter {
  final String Function(String) namer;
  final items = <Map<String, Object?>>[];

  JsonEmitter({required this.namer});

  @override
  void emit(String testName, double value) {
    const PrintEmitter().emit(testName, value);
    items.add({
      'name': namer(testName),
      'unit': "Microseconds",
      'value': value,
    });
  }
}