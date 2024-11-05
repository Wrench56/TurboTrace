/*
 * This Java source file was generated by the Gradle 'init' task.
 */

package org.example;

import io.github.wrench56.turbotrace_logger.TurboTrace;

public class App {

    public static void main(String[] args) {
        TurboTrace.init();

        String arg1 = "Hi from TurboTrace example!";
        long arg2 = 100000000000000L;
        int counter = 0;

        while (true) {
            TurboTrace.handle();
            TurboTrace.debug("The current value of counter is: {}", counter);
            TurboTrace.info("MIT rocks!");
            TurboTrace.info("Hello World {}, {}", arg1, arg2);
            otherFunction();
            TurboTrace.error("We do not like {}", returnString());
            counter++;
            sleep(10000);
        }
    }

    public static String returnString() {
        return "errors!";
    }

    public static void otherFunction() {
        TurboTrace.warn("This is a warning from {}",  "otherFunction()");
    }

    private static void sleep(long millis) {
        try {
            Thread.sleep(millis);
        } catch (InterruptedException e) {
        }
    }
}
