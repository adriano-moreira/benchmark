abstract class Actions {
  static const kNothing = 'nothing';
  static const kHelloWorld = 'hello_world';
  static const kMultiplyTable = 'multiply_table';
}

void main(List<String> arguments) {
  if (arguments.isEmpty) {
    exec(Actions.kHelloWorld);
  } else {
    exec(arguments[0]);
  }
}

final actions = <String, Function>{
  Actions.kNothing: nothing,
  Actions.kHelloWorld: helloWorld,
  Actions.kMultiplyTable: multiplyTable,
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

/// Do nothing
void nothing() {
  final value = 2 + 2;
  if (value == 5) {
    print('value is five');
  }
}

void multiplyTable() {
  final doTable = (int xLength, int yLength) {
    final table = List.filled(xLength, List.filled(yLength, 0));
    for (var x = 0; x < xLength; x++) {
      for (var y = 0; y < yLength; y++) {
        table[x][y] = (x + 1) * (y + 1);
      }
    }
    return table;
  };
  final table = doTable(1024, 1024);
  if (table[0][0] == 0) {
    print('Ops!');
  }
}
