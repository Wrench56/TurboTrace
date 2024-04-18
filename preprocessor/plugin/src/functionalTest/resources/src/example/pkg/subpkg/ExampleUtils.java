package example.pkg.subpkg;

public class ExampleUtils {
  public static final int exampleValue = 42;

  public static int returnSomeValue() {
    return 1;
  }

  public static void debug() {
    TurboTrace.debug("exampleValue: {}", exampleValue);
  }
}
