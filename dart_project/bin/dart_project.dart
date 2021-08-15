abstract class Actions {
  static const kHelloWorld = 'hello_world';
  static const kSum = 'sum';
}

void main(List<String> arguments) {
  if (arguments.isEmpty) {
    exec(Actions.kHelloWorld);
  } else {
    exec(arguments[0]);
  }
}

final actions = <String, Function>{
  Actions.kHelloWorld: helloWorld,
  Actions.kSum: sum,
};

void exec(String action) {
  final func = actions[action];
  if (func != null) {
    execWithBenchmark(action, func);
  } else {
    print('action $action not found');
  }
}

void execWithBenchmark(String action, Function func) {
  final start = DateTime.now().microsecondsSinceEpoch;
  func();
  final end = DateTime.now().microsecondsSinceEpoch;
  print('action $action elapsed ${end - start} microseconds');
}

void helloWorld() {
  print('Hello World!');
}

void sum() {
  final a = 2 + 5;
  assert(a == 5);
}
