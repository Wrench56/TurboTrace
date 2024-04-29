package io.github.wrench56;

import java.nio.ByteBuffer;

public class Utils {
  private static long prevDeltaTime = 0L;

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

  public static long calculateCurrentTimestamp() {
    return System.currentTimeMillis();
  }

  public static byte[] calculateDeltaTime() {
    long currentTimestamp = calculateCurrentTimestamp();
    byte[] timeDelta = Utils.shortToByteArray((short) (currentTimestamp - prevDeltaTime));
    prevDeltaTime = currentTimestamp;

    return timeDelta;
  }

  public static void resetDeltaTime() {
    prevDeltaTime = calculateCurrentTimestamp();
  }

  public static byte[] concatIdData(byte[] id, byte[] data) {
    byte[] combined = new byte[4 + data.length];
    System.arraycopy(id, 0, combined, 0, 4);
    System.arraycopy(data, 0, combined, 4, data.length);
    return combined;
  }

  public static byte[] longToByteArray(long value) {
    return ByteBuffer.allocate(Long.BYTES).putLong(value).array();
  }

  public static byte[] intToByteArray(int value) {
    return ByteBuffer.allocate(Integer.BYTES).putInt(value).array();
  }

  public static byte[] shortToByteArray(short value) {
    return ByteBuffer.allocate(Short.BYTES).putShort(value).array();
  }

  public static byte[] floatToByteArray(float value) {
    return ByteBuffer.allocate(Float.BYTES).putFloat(value).array();
  }

  public static byte[] doubleToByteArray(double value) {
    return ByteBuffer.allocate(Double.BYTES).putDouble(value).array();
  }
}
