package org.turbotrace;

import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.net.ServerSocket;
import java.net.Socket;

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

      /* Initialize start timestamp */
      Utils.initTime();

      if (socketHandler.verify()) {
        /* Figure out a more optimized sleep */
        try {
          Thread.sleep(10);
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
        /* TODO: Implement for other types */
        if (obj instanceof Integer) {
          outputStream.write(Utils.intToByteArray(((Integer) obj).intValue()));
        } else {
          byte[] objString = obj.toString().getBytes();
          byte[] length = Utils.castToUnsignedInt(objString.length);

          outputStream.write(length);
          outputStream.write(objString);
        }
      }
    } catch (IOException e) {
      sendData(SpecialIds.DataConcatenationFailure.id());
      return;
    }

    if (outputStream.size() == 0) return;
    socketHandler.sendMessage(outputStream.toByteArray());
  }

  /* This should be no more than ~1ms */
  public static void handle() {
    socketHandler.handleIncoming();
  }
}
