package org.turbotrace;

import java.util.regex.Matcher;
import java.util.regex.Pattern;
import org.json.JSONArray;
import org.json.JSONObject;

public class LogEntry {
  private final String package_;
  private final String logLevel;
  private final String template;

  public LogEntry(String package_, String logLevel, String template) {
    this.package_ = package_;
    this.logLevel = logLevel;
    this.template = template;
  }

  public JSONObject toJsonObject() {
    JSONObject entry = new JSONObject();

    entry.put("level", encodeLogLevel(logLevel));
    entry.put("module", package_);
    entry.put("template", template.substring(1, template.length() - 1));
    entry.put("arguments", parseArguments());

    return entry;
  }

  public JSONArray parseArguments() {
    JSONArray args = new JSONArray();
    JSONObject argEntry;

    Matcher matcher = Pattern.compile("(?<!%)%[sif]").matcher(template);
    while (matcher.find()) {
      argEntry = new JSONObject();
      /* Add more type specifiers */
      switch (matcher.group()) {
        /* String */
        case "%s":
          argEntry.put("size", 0);
          break;
        /* Number (any) */
        case "%n":
          argEntry.put("size", 0);
          break;
        /* Regular integer (32-bit) */
        case "%i":
          argEntry.put("size", 4);
          break;
        /* 64-bit integer */
        case "%li":
          argEntry.put("size", 8);
          break;
        /* Byte (number) */
        case "%b":
          argEntry.put("size", 1);
          break;
        /* Character (char) */
        case "%c":
          argEntry.put("size", 1);
          break;
        /* Error */
        default:
          argEntry.put("size", -1);
          break;
      }

      args.put(argEntry);
    }

    return args;
  }

  private int encodeLogLevel(String logLevel) {
    switch (logLevel) {
      case "debug":
        return 0;
      case "info":
        return 1;
      case "warn":
        return 2;
      case "error":
        return 3;
      case "crit":
        return 4;
      default:
        /* PRGME */
        return 5;
    }
  }
}
