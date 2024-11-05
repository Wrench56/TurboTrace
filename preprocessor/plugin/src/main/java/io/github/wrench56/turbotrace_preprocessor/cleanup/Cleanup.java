package io.github.wrench56.turbotrace_preprocessor.cleanup;

import java.io.File;
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.Comparator;
import org.gradle.api.DefaultTask;
import org.gradle.api.tasks.TaskAction;

import io.github.wrench56.turbotrace_preprocessor.Utils;

public class Cleanup extends DefaultTask {
  private static final String TEMP_FOLDER = "temp/";

  @TaskAction
  public void cleanupInit() {
    System.out.println("Starting TurboTrace cleanup...");
    Path srcDir = ((File) getProject().getExtensions().getExtraProperties().get("srcDir")).toPath();

    cleanup(srcDir);
  }

  private void cleanup(Path srcDir) {
    if (!Utils.deleteDirectory(srcDir)) {
      System.out.println("Error during cleanup: couldn't delete src/ directory");
      return;
    }

    if (!Utils.copyDirectory(srcDir.getParent().resolve(TEMP_FOLDER), srcDir)) {
      System.out.println("Error during cleanup: couldn't copy temp/ directory");
      return;
    }

    if (!Utils.deleteDirectory(srcDir.getParent().resolve(TEMP_FOLDER))) {
      System.out.println("Error during cleanup: couldn't delete temp/ directory");
      return;
    }
  }

}
