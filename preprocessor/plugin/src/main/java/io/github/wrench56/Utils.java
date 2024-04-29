package io.github.wrench56;

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
