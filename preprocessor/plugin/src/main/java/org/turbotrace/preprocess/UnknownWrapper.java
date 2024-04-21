package org.turbotrace;

/* Performant wrapper for types unknown for the preprocessor */
public class UnknownWrapper {
    private final Object unknownObj;

    public UnknownWrapper(Object obj) {
        unknownObj = obj;
    }

    public Object getWrapped() {
        return unknownObj;
    }
}
