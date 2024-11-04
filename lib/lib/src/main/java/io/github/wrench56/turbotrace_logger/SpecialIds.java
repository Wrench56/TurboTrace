package io.github.wrench56.turbotrace_logger;

public enum SpecialIds {
  KeepAlive(0x0),
  InitialTimestamp(0x1),
  PreprocessorFailure(0x2),
  DataConcatenationFailure(0x3),
  LengthOverflow(0x4),
  InternalError(0x5);

  private final int idValue;

  private SpecialIds(int idValue) {
    this.idValue = idValue;
  }

  public int id() {
    return idValue;
  }

  public byte[] byteId() {
    return Utils.castToUnsignedInt(idValue);
  }
}
