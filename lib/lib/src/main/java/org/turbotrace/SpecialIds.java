package org.turbotrace;

public enum SpecialIds {
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
