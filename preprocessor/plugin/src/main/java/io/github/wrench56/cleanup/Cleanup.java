package io.github.wrench56;

import java.io.File;
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.Comparator;
import org.gradle.api.DefaultTask;
import org.gradle.api.tasks.TaskAction;

public class Cleanup extends DefaultTask {
  private static final String TEMP_FOLDER = "temp/";

  @TaskAction
  public void cleanupInit() {
    System.out.println("Starting TurboTrace cleanup...");
    Path srcDir = ((File) getProject().getExtensions().getExtraProperties().get("srcDir")).toPath();

    cleanup(srcDir);
  }

  private void cleanup(Path srcDir) {
    if (!Utils.copyDirectory(srcDir.getParent().resolve(TEMP_FOLDER), srcDir)) {
      System.out.println("Error during cleanup: couldn't copy temp/ directory");
      return;
    }
  }

  public boolean deleteTemp(Path srcDir) {
    Path tempDir = srcDir.resolve(TEMP_FOLDER);

    try {
      Files.walk(tempDir).sorted(Comparator.reverseOrder()).map(Path::toFile).forEach(File::delete);
    } catch (IOException e) {
      return false;
    }

    /* Check whether directory was deleted */
    if (Files.exists(tempDir)) return false;

    return true;
  }
}
