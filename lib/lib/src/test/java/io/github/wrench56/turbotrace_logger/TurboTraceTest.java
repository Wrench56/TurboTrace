package io.github.wrench56.turbotrace_logger;

import static org.junit.jupiter.api.Assertions.*;

import org.junit.jupiter.api.Test;

public class TurboTraceTest {
  @Test
  public void testUnsignedCastOf0() {

    byte[] result = Utils.castToUnsignedInt(0);
    assertEquals(0, concatenateBytes(result));
  }

  @Test
  public void testUnsignedCastOf1() {
    byte[] result = Utils.castToUnsignedInt((int) 1);
    assertEquals(1, concatenateBytes(result));
  }

  @Test
  public void testUnsignedCastOfNegative1() {
    byte[] result = Utils.castToUnsignedInt((int) -1);
    assertEquals(4294967295L, concatenateBytes(result));
  }

  @Test
  public void testUnsignedCastOfNegative2() {
    byte[] result = Utils.castToUnsignedInt((int) -2);
    assertEquals(4294967294L, concatenateBytes(result));
  }

  @Test
  public void testUnsignedCastOfMaxInteger() {
    byte[] result = Utils.castToUnsignedInt((int) 4294967295L);
    assertEquals(4294967295L, concatenateBytes(result));
    assertEquals(-1, (int) 4294967295L);
  }

  private static long concatenateBytes(byte[] bytes) {
    long result = 0;
    for (int i = 0; i < bytes.length; i++) {
      result = (result << 8) | (bytes[i] & 0xFF);
    }
    return result;
  }
}
