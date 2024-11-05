package io.github.wrench56.turbotrace_preprocessor.preprocess;

import com.github.javaparser.ast.expr.ObjectCreationExpr;
import com.github.javaparser.ast.expr.Expression;
import com.github.javaparser.resolution.model.typesystem.*;
import com.github.javaparser.resolution.types.*;
import com.github.javaparser.ast.type.*;
import com.github.javaparser.ast.NodeList;
import com.github.javaparser.ast.expr.MethodCallExpr;

import javax.lang.model.type.NullType;

import org.json.JSONArray;
import org.json.JSONObject;

import io.github.wrench56.turbotrace_preprocessor.Utils;

public class LogEntry {
  private static final String UNKNOWN_WRAPPER_CLASS = "UnknownWrapper";

  private final String package_;
  private final String logLevel;
  private final String template;
  private final MethodCallExpr methodCallExpr;

  public LogEntry(
      String package_, String logLevel, String template, MethodCallExpr methodCallExpr) {
    this.package_ = package_;
    this.logLevel = logLevel;
    this.template = template;
    this.methodCallExpr = methodCallExpr;
  }

  public JSONObject toJsonObject() {
    JSONObject entry = new JSONObject();

    entry.put("level", encodeLogLevel(logLevel));
    entry.put("module", package_);
    entry.put("arguments", parseArguments(entry));

    return entry;
  }

  public JSONArray parseArguments(JSONObject entry) {
    JSONArray args = new JSONArray();
    String template = this.template;
    int argNumber = 0;

    ResolvedType argType;
    for (Expression arg : methodCallExpr.getArguments()) {
      /* Skip template string */
      if (argNumber == 0) {
        ++argNumber;
        continue;
      }

      if (arg.isLambdaExpr()) {
        /* This is a problem */
        continue;
      }

      argType = arg.calculateResolvedType();
      if (argType instanceof LazyType) {
        String value = arg.toString();
        template = Utils.replaceNthOccurrence(template, "{}", argNumber - 1, value.substring(1, value.length() - 1));
      } else if (argType instanceof ResolvedPrimitiveType) {
        /* Primitives */
        args.put(((ResolvedPrimitiveType) argType).describe());
      } else if (argType instanceof ReferenceType) {
        /* References (Strings) */
        args.put(((ReferenceType) argType).resolve().describe());
      } else if (argType instanceof NullType) {
        /* Ignore and remove from template */
        template = Utils.replaceNthOccurrence(template, "{}", argNumber - 1, "");
      } else if (argType instanceof ResolvedVoidType) {
        /* TODO: Find out whether we need this */
      } else {
        System.out.println("Warning: Unknown wrapper used for argument: " + arg);
        wrapWithUnknownWrapper(arg);
        args.put("str");
      }

      ++argNumber;
    }

    /* Remove escaped "-s */
    entry.put("template", template.substring(1, template.length() - 1));
    return args;

  }

  /* TODO: Use this */
  private ObjectCreationExpr wrapWithUnknownWrapper(Expression arg) {
    ObjectCreationExpr objectCreationExpr = new ObjectCreationExpr();
    objectCreationExpr.setType(UNKNOWN_WRAPPER_CLASS);
    objectCreationExpr.setArguments(NodeList.nodeList(arg));
    return objectCreationExpr;
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
