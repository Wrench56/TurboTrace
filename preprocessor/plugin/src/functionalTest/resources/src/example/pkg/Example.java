package example.pkg;

import example.pkg.subpkg.ExampleUtils;

public class Example {
  int value = 0;

  void test() {
    TurboTrace.info("Hello World: {} {}", ExampleUtils.returnSomeValue(), value);
  }

  void callUtilsDebug() {
    TurboTrace.info("Calling ExampleUtils.debug()");
    ExampleUtils.debug();
  }
}
