package org.turbotrace;

import org.gradle.testfixtures.ProjectBuilder;
import org.gradle.api.Project;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.BeforeEach;
import static org.junit.jupiter.api.Assertions.*;

class PreprocessorPluginTest {
    private Project project;

    @BeforeEach
    void setUp() {
        project = ProjectBuilder.builder().build();
        project.getPlugins().apply("org.turbotrace.preprocess");
    }

    @Test
    void pluginRegistersPreprocess() {
        assertNotNull(project.getTasks().findByName("preprocess"));
    }

    @Test
    void pluginRegistersCleanup() {
        assertNotNull(project.getTasks().findByName("cleanup"));
    }
}
