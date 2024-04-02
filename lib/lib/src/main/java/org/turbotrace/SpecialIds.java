package org.turbotrace;

public enum SpecialIds {
  PreprocessorFailure(0x1),
  DataConcatenationFailure(0x2),
  InternalError(0x3),
  InitialTimestamp(0x4);

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
