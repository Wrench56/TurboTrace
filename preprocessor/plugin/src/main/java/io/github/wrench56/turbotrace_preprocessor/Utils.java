package io.github.wrench56.turbotrace_preprocessor;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.SimpleFileVisitor;
import java.nio.file.StandardCopyOption;
import java.nio.file.attribute.BasicFileAttributes;
import java.nio.file.FileVisitResult;

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

  public static boolean deleteDirectory(Path directory) {
    try {
      Files.walkFileTree(directory, new SimpleFileVisitor<>() {
        @Override
        public FileVisitResult visitFile(Path file, BasicFileAttributes attrs) throws IOException {
          Files.delete(file);
          return FileVisitResult.CONTINUE;
        }

        @Override
        public FileVisitResult postVisitDirectory(Path dir, IOException exc) throws IOException {
          Files.delete(dir);
          return FileVisitResult.CONTINUE;
        }
      });
      return true;
    } catch (IOException e) {
      return false;
    }
  }

  /*
   * Source:
   * https://stackoverflow.com/questions/3976616/how-to-find-nth-occurrence-of-
   * character-in-a-string
   */
  public static int ordinalIndexOf(String str, String substr, int n) {
    int pos = str.indexOf(substr);
    while (--n > 0 && pos != -1)
      pos = str.indexOf(substr, pos + 1);
    return pos;
  }

  public static String replaceNthOccurrence(String str, String substr, int n, String replacement) {
    int index = ordinalIndexOf(str, substr, n);
    StringBuffer strBuff = new StringBuffer(str);
    strBuff.replace(index, index + substr.length(), replacement);
    return strBuff.toString();
  }
}
