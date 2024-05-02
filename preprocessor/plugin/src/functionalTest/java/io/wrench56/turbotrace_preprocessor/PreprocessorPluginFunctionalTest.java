package io.github.wrench56.turbotrace_preprocessor;

import static org.junit.jupiter.api.Assertions.*;

import java.io.File;
import java.io.FileWriter;
import java.io.IOException;
import java.io.Writer;
import java.nio.file.Files;
import java.nio.file.Path;
import org.gradle.testkit.runner.BuildResult;
import org.gradle.testkit.runner.GradleRunner;
import org.junit.jupiter.api.Order;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.TestMethodOrder;
import org.junit.jupiter.api.MethodOrderer;
import org.junit.jupiter.api.io.TempDir;

@TestMethodOrder(MethodOrderer.OrderAnnotation.class)
class PreprocessorPluginFunctionalTest {
  private static final Path EXAMPLE_SRC = new File("src/functionalTest/resources/src/").toPath();
  private static final File EXPECTED_LOGTYPES = new File("src/functionalTest/resources/results/logtypes.json");
  private static final String TEMP_DIR = "temp/";

  @TempDir
  static File projectDir;
  private static Path EXAMPLE_LOC;

  private File getBuildFile() {
    return new File(projectDir, "build.gradle");
  }

  private File getSettingsFile() {
    return new File(projectDir, "settings.gradle");
  }

  private boolean createFolders() {
    if (!new File(projectDir, "src").mkdir()) {
      System.out.println("Error during functional testing: couldn't create src directory");
      return false;
    }
    if (!new File(projectDir, "build").mkdir()) {
      System.out.println("Error during functional testing: couldn't create build directory");
      return false;
    }

    return true;
  }

  @Test
  @Order(1)
  void canRunTask() throws IOException {
    EXAMPLE_LOC = projectDir.toPath().resolve("src/");

    /* Fail early on errors */
    if (!createFolders())
      assertTrue(false);
    if (!Utils.copyDirectory(EXAMPLE_SRC, EXAMPLE_LOC))
      assertTrue(false);

    writeString(getSettingsFile(), "");
    writeString(getBuildFile(), "plugins {" + "  id('io.github.wrench56.turbotrace-preprocessor')" + "}");

    GradleRunner runner = GradleRunner.create();
    runner.forwardOutput();
    runner.withPluginClasspath();
    runner.withArguments("preprocess");
    runner.withProjectDir(projectDir);
    BuildResult result = runner.build();

    /* Preprocessor starts */
    assertTrue(result.getOutput().contains("Starting TurboTrace preprocessor..."));
  }

  @Test
  @Order(2)
  void logtypesCreated() {
    /* JSON logtypes created */
    assertTrue(Files.exists(EXAMPLE_LOC.resolve("logtypes.json")));
  }

  @Test
  @Order(3)
  void logtypesCorrect() throws IOException {
    assertTrue(
        Utils.areFilesEqual(
            EXAMPLE_LOC.resolve("logtypes.json").toFile(), EXPECTED_LOGTYPES));
  }

  @Test
  @Order(4)
  void canRunCleanup() {
    GradleRunner runner = GradleRunner.create();
    runner.forwardOutput();
    runner.withPluginClasspath();
    runner.withArguments("cleanup");
    runner.withProjectDir(projectDir);
    BuildResult result = runner.build();

    assertTrue(result.getOutput().contains("Starting TurboTrace cleanup..."));
  }

  @Test
  @Order(5)
  void tempDeleted() {
    assertTrue(Files.exists(projectDir.toPath().resolve(TEMP_DIR)));
  }

  @Test
  @Order(6)
  void srcRestored() {
    assertTrue(Utils.areDirectoriesEqual(EXAMPLE_SRC.resolve("example/pkg").toFile(), EXAMPLE_LOC.toFile()));
  }

  private void writeString(File file, String string) throws IOException {
    try (Writer writer = new FileWriter(file)) {
      writer.write(string);
    }
  }
}
