package org.turbotrace;

import com.github.javaparser.StaticJavaParser;
import com.github.javaparser.ParserConfiguration;
import com.github.javaparser.ast.CompilationUnit;
import com.github.javaparser.symbolsolver.JavaSymbolSolver;
import com.github.javaparser.symbolsolver.utils.SymbolSolverCollectionStrategy;
import com.github.javaparser.symbolsolver.resolution.typesolvers.*;
import java.io.File;
import java.io.FileWriter;
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.List;
import org.gradle.api.DefaultTask;
import org.gradle.api.tasks.TaskAction;
import org.json.JSONObject;

public class PreprocessTask extends DefaultTask {
  private static final String TEMP_FOLDER = "temp/";
  private static final String SRC_FOLDER = "src/";
  private static final String BUILD_FOLDER = "build/";
  private static final String JSON_PATH = "logtypes.json";

  private static JSONObject json;

  @TaskAction
  public void preprocessTask() {
    System.out.println("Starting TurboTrace preprocessor...");
    Path srcDir = ((File) getProject().getExtensions().getExtraProperties().get("srcDir")).toPath();
    Path buildDir =
        ((File) getProject().getExtensions().getExtraProperties().get("buildDir")).toPath();

    preprocessInit(srcDir, buildDir);
    preprocess(srcDir);
  }

  private void preprocessInit(Path srcDir, Path buildDir) {
    /* Create directories */
    if (!createDirectory(srcDir, "Error during preprocessing: couldn't create srcDir")) return;
    if (!createDirectory(buildDir, "Error during preprocessing: couldn't create buildDir")) return;
    if (!createDirectory(
        srcDir.getParent().resolve(TEMP_FOLDER),
        "Error during preprocessing: couldn't create tempDir")) return;

    /* Copy the original source code */
    if (!Utils.copyDirectory(srcDir, srcDir.getParent().resolve(TEMP_FOLDER))) {
      System.out.println("Error during preprocessing: couldn't copy srcDir");
      return;
    }

    /* Create Java parsers */
    createJavaParser(srcDir);

    /* Setup json */
    PreprocessTask.json = new JSONObject();
  }

  private void preprocess(Path srcDir) {
    /* Collect all the source files recusively */
    List<Path> files = collectFiles(srcDir);
    if (files.size() == 0) return;

    files.forEach(PreprocessTask::processFile);

    /* Save JSON */
    if (!saveJsonToFile(PreprocessTask.json, srcDir.resolve(JSON_PATH))) return;
  }

  private boolean createDirectory(Path directory, String errorMsg) {
    try {
      Files.createDirectories(directory);
    } catch (IOException e) {
      System.out.println(errorMsg);
      return false;
    }

    return true;
  }

  private List<Path> collectFiles(Path srcDir) {
    try {
      return Files.walk(srcDir)
          .filter(Files::isRegularFile)
          .filter(p -> p.toString().endsWith(".java"))
          .toList();
    } catch (IOException e) {
      System.out.println("Error during preprocessing: couldn't collect all source files");
      return new ArrayList<Path>();
    }
  }

  private void createJavaParser(Path srcDir) {
    CombinedTypeSolver typeSolver = new CombinedTypeSolver();

    /* Source directory and subdirectories */
    typeSolver.add(new JavaParserTypeSolver(srcDir.toFile()));

    /* Java built-ins */
    typeSolver.add(new ReflectionTypeSolver(false));

    StaticJavaParser.setConfiguration(new ParserConfiguration().setSymbolResolver(new JavaSymbolSolver(typeSolver)));
  }

  private static void processFile(Path path) {
    try {
      CompilationUnit cu = StaticJavaParser.parse(path);
      cu.accept(new TurboTraceLogVisitor(path.toString(), PreprocessTask.json), null);
    } catch (IOException e) {
      System.out.println(
          String.format("Error during preprocessing: couldn't process file \"%s\"", path));
    }
  }

  private boolean saveJsonToFile(JSONObject jsonObject, Path file) {
    try (FileWriter fileWriter = new FileWriter(file.toFile())) {
      fileWriter.write(jsonObject.toString(4));
    } catch (IOException e) {
      System.out.println("Error during preprocessing: couldn't save log types");
      return false;
    }

    return true;
  }
}
