package org.turbotrace;

import java.nio.ByteBuffer;

public class Utils {
  private static long startTimestamp;

  public static byte[] castToUnsignedInt(long value) {
    byte[] byteArray = new byte[4];
    /* Ensure it's not longer than 4 bytes */
    if (value <= 4294967295L) {
      /* Cast to unsigned int */
      byteArray[0] = (byte) (value >>> 24);
      byteArray[1] = (byte) (value >>> 16);
      byteArray[2] = (byte) (value >>> 8);
      byteArray[3] = (byte) value;
    }

    return byteArray;
  }

  public static boolean byteArraysEqual(byte[] array1, byte[] array2) {
    for (int i = 0; i < array1.length; i++) {
      if (array1[i] != array2[i]) {
        return false;
      }
    }
    return true;
  }

  public static void initTime() {
    startTimestamp = System.currentTimeMillis();
  }

  public static long getInitialTime() {
    return startTimestamp;
  }

  public static byte[] calculateTimestamp() {
    long deltaTimestamp = System.currentTimeMillis() - startTimestamp;

    return Utils.castToUnsignedInt(deltaTimestamp);
  }

  public static byte[] longToByteArray(long value) {
    return ByteBuffer.allocate(8).putLong(value).array();
  }

  public static byte[] intToByteArray(int value) {
    return ByteBuffer.allocate(4).putInt(value).array();
  }
}
