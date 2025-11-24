// app/build.gradle.kts

// Aplicar los plugins definidos en settings.gradle.kts
plugins {
    id("org.jetbrains.kotlin.jvm")
    id("org.jetbrains.kotlin.plugin.serialization")
}

group = "org.example"
version = "1.0-SNAPSHOT"

// Repositorios ya definidos en settings.gradle.kts, pero se pueden repetir aquí
repositories {
    mavenCentral()
}

dependencies {
    // Dependencias básicas de Kotlin
    implementation("org.jetbrains.kotlin:kotlin-stdlib")

    // --- Dependencias para el Servidor Rust (Ktor y Serialización) ---

    // Kotlin Coroutines
    implementation("org.jetbrains.kotlinx:kotlinx-coroutines-core:1.7.3")

    // Kotlinx Serialization
    implementation("org.jetbrains.kotlinx:kotlinx-serialization-json:1.6.0")

    // Ktor Client (Core y Motor)
    implementation("io.ktor:ktor-client-core:2.3.6")
    implementation("io.ktor:ktor-client-cio:2.3.6")

    // Ktor para serialización JSON
    implementation("io.ktor:ktor-client-content-negotiation:2.3.6")
    implementation("io.ktor:ktor-serialization-kotlinx-json:2.3.6")

    // Si tu módulo 'app' depende de 'utils'
    // implementation(project(":utils"))

    // SOLUCIÓN: Agrega la implementación de Logback para que SLF4J pueda registrar
    implementation("ch.qos.logback:logback-classic:1.4.14")
}

tasks.test {
    useJUnitPlatform()
}