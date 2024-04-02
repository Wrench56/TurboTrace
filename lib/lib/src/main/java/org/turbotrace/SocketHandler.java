package org.turbotrace;

import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;
import java.net.Socket;

public class SocketHandler {
  private enum MessageCodes {
    ACK(new byte[] {'A', 'C', 'K'}), // Acknowledge
    ERR(new byte[] {'E', 'R', 'R'}), // Error on the other side
    VER(new byte[] {'V', 'E', 'R'}), // Verification
    ITS(new byte[] {'I', 'T', 'S'}), // Initial timestamp
    IER(new byte[] {'I', 'E', 'R'}), // Internal error
    NOP(new byte[] {'N', 'O', 'P'}); // No operation

    private final byte[] byteArray;

    MessageCodes(byte[] byteArray) {
      this.byteArray = byteArray;
    }

    public static MessageCodes fromByteArray(byte[] array) {
      for (MessageCodes enumValue : MessageCodes.values()) {
        if (Utils.byteArraysEqual(enumValue.byteArray, array)) return enumValue;
      }
      return MessageCodes.IER;
    }
  };

  private static final byte[] TT_INIT = {'T', 'T', 'i', 'n', 'i', 't'};

  private OutputStream outputStream;
  private InputStream inputStream;

  public SocketHandler(Socket client) {
    try {
      this.inputStream = client.getInputStream();
      this.outputStream = client.getOutputStream();
    } catch (IOException e) {
      this.inputStream = null;
      this.outputStream = null;
    }
  }

  public boolean verify() {
    if (inputStream == null) return false;
    int iters = 0;

    /* Send verification message */
    MessageCodes msg = recvMessage();
    while (msg != MessageCodes.ERR) {
      switch (msg) {
        case VER:
          if (!sendMessage(TT_INIT)) return false;
          break;
        case ACK:
          return true;
        default:
          break;
      }

      try {
        Thread.sleep(10);
      } catch (InterruptedException e) {
        return false;
      }

      /* Fail at 1 second */
      if (iters == 100) return false;
      ++iters;

      msg = recvMessage();
    }

    return false;
  }

  public boolean sendMessage(byte[] message) {
    try {
      outputStream.write(message);
      outputStream.flush();
    } catch (IOException e) {
      return false;
    }
    return true;
  }

  public MessageCodes recvMessage() {
    try {
      if (inputStream.available() < 3) return MessageCodes.NOP;
    } catch (IOException e) {
      return MessageCodes.IER;
    }

    byte[] buffer = new byte[3];
    try {
      int count = inputStream.read(buffer, 0, buffer.length);

      if (count == -1) return MessageCodes.IER;

      /* TODO: Handle this better */
      if (count != 3) return MessageCodes.IER;
    } catch (IOException e) {
      return MessageCodes.IER;
    }

    return MessageCodes.fromByteArray(buffer);
  }

  public int handleIncoming() {
    int handled = 0;
    while (true) {
      switch (recvMessage()) {
        case NOP:
          return handled;
        case ITS:
          sendMessage(Utils.calculateTimestamp());
          break;
        case IER:
          sendMessage(SpecialIds.InternalError.byteId());
          break;

        default:
          break;
      }

      ++handled;
    }
  }
}
