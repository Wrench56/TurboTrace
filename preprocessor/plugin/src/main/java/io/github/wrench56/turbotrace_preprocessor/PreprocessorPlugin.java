package io.github.wrench56.turbotrace_preprocessor;

import org.gradle.api.Plugin;
import org.gradle.api.Project;

import io.github.wrench56.turbotrace_preprocessor.preprocess.PreprocessTask;
import io.github.wrench56.turbotrace_preprocessor.cleanup.Cleanup;

public class PreprocessorPlugin implements Plugin<Project> {
  @Override
  public void apply(Project project) {
    globalVariables(project);
    registerTasks(project);
  }

  private void globalVariables(Project project) {
    project.getExtensions().getExtraProperties().set("srcDir", project.file("src"));
    project.getExtensions().getExtraProperties().set("buildDir", project.file("build"));
  }

  private void registerTasks(Project project) {
    project.getTasks().register("preprocess", PreprocessTask.class);
    project.getTasks().register("cleanup", Cleanup.class);
  }
}
