plugins {
    kotlin("jvm") version "1.3.61"
    java
    application
}

group = "logica"
version = "NaN"

repositories {
    mavenCentral()
    jcenter()
}

dependencies {
    implementation(kotlin("stdlib"))

    testImplementation("junit", "junit", "4.12")
    testImplementation(kotlin("test-junit"))
}

configure<JavaPluginConvention> {
    sourceCompatibility = JavaVersion.VERSION_13
}

application {
    mainClassName = "logica.KtMain"
}