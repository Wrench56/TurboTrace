package org.turbotrace;

import java.io.File;
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.StandardCopyOption;
import java.util.ArrayList;
import java.util.List;

import org.apache.commons.io.FileUtils;

public class Utils {
  public static boolean copyDirectory(Path sourceDir, Path destinationDir) {
    try {
      _copyDirectory(sourceDir, destinationDir);
    } catch (RuntimeException e) {
      return false;
    } catch (IOException e) {
      return false;
    }

    return true;
  }

  private static void _copyDirectory(Path sourceDir, Path destinationDir)
      throws RuntimeException, IOException {
    Files.walk(sourceDir)
        .forEach(
            source -> {
              try {
                Path destination = destinationDir.resolve(sourceDir.relativize(source));
                Files.copy(source, destination, StandardCopyOption.REPLACE_EXISTING);
              } catch (IOException e) {
                /* Lambda magic... */
                throw new RuntimeException(e);
              }
            });
  }

  public static boolean areFilesEqual(File file1, File file2) {
    try {
      return FileUtils.contentEqualsIgnoreEOL(
          file1, file2, "UTF-8");
    } catch (IOException e) {
      return false;
    }
  }

  private static List<Path> collectFiles(Path srcDir) {
    try {
      return Files.walk(srcDir)
          .filter(Files::isRegularFile)
          .filter(p -> !p.toString().endsWith("logtypes.json"))
          .toList();
    } catch (IOException e) {
      return new ArrayList<Path>();
    }
  }

  public static boolean areDirectoriesEqual(File dir1, File dir2) {
    /* Fail if File-s are not directories */
    if (!dir1.isDirectory() || !dir2.isDirectory()) {
      return false;
    }

    List<Path> files1 = collectFiles(dir1.toPath());
    List<Path> files2 = collectFiles(dir2.toPath());

    /* Fail if directories have different number of files */
    if (files1.size() != files2.size()) {
      return false;
    }

    for (int i = 0; i < files1.size(); i++) {
      if (!areFilesEqual(files1.get(i).toFile(), files2.get(i).toFile())) {
        return false;
      }
    }

    return true;
  }
}
