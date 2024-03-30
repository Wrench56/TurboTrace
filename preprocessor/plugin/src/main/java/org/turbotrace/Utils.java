package org.turbotrace;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.StandardCopyOption;

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
}
