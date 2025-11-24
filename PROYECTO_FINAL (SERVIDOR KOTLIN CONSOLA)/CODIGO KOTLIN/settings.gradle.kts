// settings.gradle.kts

pluginManagement {
    // Definir dónde encontrar los plugins
    repositories {
        gradlePluginPortal()
        mavenCentral()
    }
    // Definir las versiones de los plugins de Kotlin que usarás en tus módulos
    // Puedes colocar aquí los plugins comunes como el de serialización y JVM.
    plugins {
        id("org.jetbrains.kotlin.jvm") version "1.9.20"
        id("org.jetbrains.kotlin.plugin.serialization") version "1.9.20"
    }
}

dependencyResolutionManagement {
    @Suppress("UnstableApiUsage")
    repositories {
        mavenCentral()
    }
}

plugins {
    // Se mantiene el plugin de Foojay Toolchains
    id("org.gradle.toolchains.foojay-resolver-convention") version "0.8.0"
}

// Incluir los módulos subproyectos
include(":app")
include(":utils")

rootProject.name = "Benchmark"