import org.jetbrains.kotlin.gradle.tasks.KotlinCompile

plugins {
  kotlin("jvm") version "1.4.0"
  application
}

val kotlinVersion = "1.4.0"
val junitJupiterVersion = "5.6.2"

group = "logica"
version = "NaN"

repositories {
  mavenCentral()
  jcenter()
}

val launcherClassName = "logica.MainKt"

dependencies {
  implementation(kotlin("stdlib"))

  testImplementation(kotlin("test-junit"))
  testImplementation("org.junit.jupiter:junit-jupiter:$junitJupiterVersion")
}

application {
  mainClassName = "logica.Main"
}

tasks.withType<KotlinCompile> {
  kotlinOptions.jvmTarget = "13"
}

tasks.getByName<Test>("test") {
  useJUnitPlatform()
}

val compileKotlin: KotlinCompile by tasks
compileKotlin.kotlinOptions.jvmTarget = "13"

tasks {
  test {
    useJUnitPlatform()
  }
}

