import org.jetbrains.kotlin.gradle.tasks.KotlinCompile

val kotlinVersion = "1.3.72"
val junitJupiterVersion = "5.6.0"

plugins {
  kotlin("jvm") version "1.3.72"
  application
}

group = "logica"
version = "NaN"

application {
  mainClassName = "logica.MainKt"
}

repositories {
  mavenCentral()
  jcenter()
}

dependencies {
  implementation(kotlin("stdlib"))

  testImplementation(kotlin("test-junit"))
  testImplementation("org.junit.jupiter:junit-jupiter:$junitJupiterVersion")
}

tasks.withType<KotlinCompile> {
  kotlinOptions.jvmTarget = "13"
}

tasks.getByName<Test>("test") {
  useJUnitPlatform()
}
