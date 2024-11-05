package io.github.wrench56.turbotrace_preprocessor;

import org.gradle.api.Plugin;
import org.gradle.api.Project;
import org.gradle.api.tasks.TaskProvider;
import org.gradle.api.tasks.compile.JavaCompile;

import io.github.wrench56.turbotrace_preprocessor.preprocess.PreprocessTask;
import io.github.wrench56.turbotrace_preprocessor.cleanup.Cleanup;

public class PreprocessorPlugin implements Plugin<Project> {
  @Override
  public void apply(Project project) {
    globalVariables(project);

    TaskProvider<PreprocessTask> preprocessTask = project.getTasks().register("preprocess", PreprocessTask.class);
    TaskProvider<Cleanup> cleanupTask = project.getTasks().register("cleanup", Cleanup.class);

    project.getTasks().withType(JavaCompile.class).configureEach(compileJava -> {
      compileJava.doFirst(task -> {
        project.getLogger().lifecycle("Preprocessing with TurboTrace...");
        preprocessTask.get().getActions().forEach(action -> action.execute(preprocessTask.get()));
      });
    });

    project.getTasks().withType(JavaCompile.class).configureEach(compileJava -> {
      compileJava.doLast(task -> {
        project.getLogger().lifecycle("Cleaning up TurboTrace...");
        cleanupTask.get().getActions().forEach(action -> action.execute(cleanupTask.get()));
      });
    });

  }

  private void globalVariables(Project project) {
    project.getExtensions().getExtraProperties().set("srcDir", project.file("src"));
    project.getExtensions().getExtraProperties().set("buildDir", project.file("build"));
  }
}
