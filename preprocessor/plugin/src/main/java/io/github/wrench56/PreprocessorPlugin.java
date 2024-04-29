package io.github.wrench56;

import org.gradle.api.Plugin;
import org.gradle.api.Project;

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
