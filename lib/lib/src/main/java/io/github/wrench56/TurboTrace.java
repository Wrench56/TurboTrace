package io.github.wrench56;

import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.net.ServerSocket;
import java.net.Socket;
import java.nio.charset.StandardCharsets;

public class TurboTrace {
  private static final int PORT = 40025;

  private static ServerSocket server;
  private static Socket socket;
  private static SocketHandler socketHandler;

  public static void init() {
    /* Create socket */
    try {
      server = new ServerSocket(PORT);

      /* TODO: Blocking call, fix this */
      socket = server.accept();
      socketHandler = new SocketHandler(socket);

      if (socketHandler.verify()) {
        /* TODO: Figure out a more optimized sleep */
        try {
          Thread.sleep(50);
        } catch (InterruptedException e) {
        }
        socketHandler.handleIncoming();
      }
    } catch (IOException e) {
      /* TODO: Handle this better */
      e.printStackTrace();
    }
  }

  /* Preprocessor did not run */
  public static void log(String template, Object... args) {
    sendData(SpecialIds.PreprocessorFailure.id());
  }

  public static void log(long id, Object... args) {
    sendData(id, args);
  }

  /* Autobox any primitive object */
  private static void sendData(long id, Object... args) {
    ByteArrayOutputStream outputStream = new ByteArrayOutputStream();
    try {
      outputStream.write(Utils.castToUnsignedInt(id));
      for (Object obj : args) {
        /* TODO: Implement for arrays */
        if (obj instanceof Integer) {
          outputStream.write(Utils.intToByteArray(((Integer) obj).intValue()));
        } else if (obj instanceof Long) {
          outputStream.write(Utils.longToByteArray(((Long) obj).longValue()));
        } else if (obj instanceof Short) {
          outputStream.write(Utils.shortToByteArray(((Short) obj).shortValue()));
        } else if (obj instanceof Boolean) {
          outputStream.write((byte) (((Boolean) obj).booleanValue() ? 1 : 0));
        } else if (obj instanceof Float) {
          outputStream.write(Utils.floatToByteArray(((Float) obj).floatValue()));
        } else if (obj instanceof Double) {
          outputStream.write(Utils.doubleToByteArray(((Double) obj).doubleValue()));
        } else if (obj instanceof Byte) {
          outputStream.write(((Byte) obj).byteValue());
        } else if (obj instanceof Character) {
          outputStream.write(((Character) obj).toString().getBytes(StandardCharsets.UTF_16BE));
        } else {
          byte[] objString = obj.toString().getBytes();
          int length = objString.length;

          if (length < 255)
            /* Send u8 length */
            outputStream.write((byte) (length & 0xFF));
          else if (length < 65534) {
            /* Send u16 length */
            outputStream.write((byte) 0);
            outputStream.write(Utils.shortToByteArray((short) (length & 0xFFFF)));
          } else {
            sendData(SpecialIds.LengthOverflow.id());
            return;
          }

          outputStream.write(objString);
        }
      }
    } catch (IOException e) {
      sendData(SpecialIds.DataConcatenationFailure.id());
      return;
    }

    if (outputStream.size() == 0)
      return;
    socketHandler.sendMessage(outputStream.toByteArray());
  }

  /* This should be no more than ~1ms */
  /* TODO: Disconnect if client closed */
  public static void handle() {
    socketHandler.handleIncoming();
    socketHandler.handleKeepAlive();
  }
}
