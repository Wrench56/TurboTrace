package example.pkg;

public class Example {
  int value = 0;

  void test() {
    TurboTrace.info("Hello World: %s %i", ExampleUtils.returnSomeValue(), value);
  }

  void callUtilsDebug() {
    TurboTrace.info("Calling ExampleUtils.debug()");
    ExampleUtils.debug();
  }
}
