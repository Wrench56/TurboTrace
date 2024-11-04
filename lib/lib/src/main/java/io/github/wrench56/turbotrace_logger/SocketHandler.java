package io.github.wrench56.turbotrace_logger;

import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;
import java.net.Socket;

public class SocketHandler {
  private enum MessageCodes {
    ACK(new byte[] { 'A', 'C', 'K' }), // Acknowledge
    ERR(new byte[] { 'E', 'R', 'R' }), // Error on the other side
    VER(new byte[] { 'V', 'E', 'R' }), // Verification
    ITS(new byte[] { 'I', 'T', 'S' }), // Initial timestamp
    IER(new byte[] { 'I', 'E', 'R' }), // Internal error
    NOP(new byte[] { 'N', 'O', 'P' }); // No operation

    private final byte[] byteArray;

    MessageCodes(byte[] byteArray) {
      this.byteArray = byteArray;
    }

    public static MessageCodes fromByteArray(byte[] array) {
      for (MessageCodes enumValue : MessageCodes.values()) {
        if (Utils.byteArraysEqual(enumValue.byteArray, array))
          return enumValue;
      }
      return MessageCodes.IER;
    }
  };

  private static final byte[] TT_INIT = { 'T', 'T', 'i', 'n', 'i', 't' };

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
    if (inputStream == null)
      return false;
    int iters = 0;

    /* Send verification message */
    MessageCodes msg = recvMessage();
    while (msg != MessageCodes.ERR) {
      switch (msg) {
        case VER:
          if (!sendRawMessage(TT_INIT))
            return false;
          break;
        case ACK:
          return true;
        default:
          break;
      }

      try {
        Thread.sleep(50);
      } catch (InterruptedException e) {
        return false;
      }

      /* Fail at 1 second */
      if (iters == 100)
        return false;
      ++iters;

      msg = recvMessage();
    }

    return false;
  }

  public boolean sendMessage(byte[] message) {
    byte[] combined = new byte[2 + message.length];
    System.arraycopy(Utils.calculateDeltaTime(), 0, combined, 0, 2);
    System.arraycopy(message, 0, combined, 2, message.length);

    return sendRawMessage(combined);
  }

  private boolean sendRawMessage(byte[] message) {
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
      if (inputStream.available() < 3)
        return MessageCodes.NOP;
    } catch (IOException e) {
      return MessageCodes.IER;
    }

    byte[] buffer = new byte[3];
    try {
      int count = inputStream.read(buffer, 0, buffer.length);

      if (count == -1)
        return MessageCodes.IER;
      if (count != 3)
        return MessageCodes.IER;
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
          sendInitialTimestamp();
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

  private boolean sendInitialTimestamp() {
    byte[] combined = new byte[14];
    byte[] message = Utils.concatIdData(
        SpecialIds.InitialTimestamp.byteId(),
        Utils.longToByteArray(System.currentTimeMillis()));
    Utils.resetDeltaTime();
    System.arraycopy(new byte[] { 0, 0 }, 0, combined, 0, 2);
    System.arraycopy(message, 0, combined, 2, 12);

    return sendRawMessage(combined);
  }

  public short handleKeepAlive() {
    short deltaTime = Utils.calculateDeltaTimeShort();

    /* Reset delta time and send ITS if we are close to overflow */
    if (deltaTime >= 60000) {
      sendMessage(SpecialIds.KeepAlive.byteId());
    }

    return deltaTime;
  }
}
