package org.turbotrace;

import com.github.javaparser.ast.Node;
import com.github.javaparser.ast.body.*;
import com.github.javaparser.ast.expr.*;
import com.github.javaparser.ast.visitor.VoidVisitorAdapter;
import java.util.List;
import java.util.regex.Matcher;
import java.util.regex.Pattern;
import org.json.JSONObject;

public class TurboTraceLogVisitor extends VoidVisitorAdapter<Void> {
  private static int id = 0x0;
  private static JSONObject json;

  private String filePath;
  private String logLevel;
  private String packageString;

  TurboTraceLogVisitor(String filePath, JSONObject json) {
    this.filePath = filePath;
    TurboTraceLogVisitor.json = json;
  }

  @Override
  public void visit(MethodCallExpr methodCallExpr, Void arg) {
    super.visit(methodCallExpr, arg);

    /* Check if method is TurboTrace.* */
    if (!(methodCallExpr.getScope().isPresent()
        && methodCallExpr.getScope().get().toString().equals("TurboTrace"))) return;

    logLevel = methodCallExpr.getNameAsString();

    MethodDeclaration methodDeclaration = findEnclosingMethod(methodCallExpr);
    if (methodDeclaration != null) {
      ClassOrInterfaceDeclaration classDeclaration = findEnclosingClass(methodDeclaration);
      if (classDeclaration != null) {
        packageString =
            createModuleString(
                filePath, classDeclaration.getNameAsString(), methodDeclaration.getNameAsString());
        process(methodCallExpr);
      }
    }
  }

  private MethodDeclaration findEnclosingMethod(Node node) {
    while (node != null && !(node instanceof MethodDeclaration)) {
      node = node.getParentNode().orElse(null);
    }
    return (MethodDeclaration) node;
  }

  private ClassOrInterfaceDeclaration findEnclosingClass(Node node) {
    while (node != null && !(node instanceof ClassOrInterfaceDeclaration)) {
      node = node.getParentNode().orElse(null);
    }
    return (ClassOrInterfaceDeclaration) node;
  }

  private boolean process(MethodCallExpr methodCallExpr) {
    methodCallExpr.setName("log");

    List<Expression> args = methodCallExpr.getArguments();
    if (!args.get(0).isLiteralStringValueExpr()) {
      System.out.println("Error during preprocessing: first argument is not a template string");
      return false;
    }

    if (args.size() - 1 != countArgumentSpecifiers(args.get(0).toString())) {
      System.out.println(
          "Error during preprocessing: argument specifiers and arguments do not equal each other");
      return false;
    }

    /* Save log type */
    LogEntry entry = new LogEntry(packageString, logLevel, args.get(0).toString());
    json.put(Integer.toString(id), entry.toJsonObject());

    /* Inject id into method as argument */
    methodCallExpr.setArgument(0, new IntegerLiteralExpr(Integer.toString(id++)));

    return true;
  }

  private String createModuleString(String filename, String className, String methodName) {
    return filename.split("src[/\\\\]")[1].replaceAll("/", "::").replaceAll("\\\\", "::")
        + "::"
        + className
        + "::"
        + methodName;
  }

  private int countArgumentSpecifiers(String str) {
    /* Ignore any escaped %-s (%%) and non-valid %-s */
    Matcher matcher = Pattern.compile("(?<!%)%[sif]").matcher(str);
    return (int) matcher.results().count();
  }
}
