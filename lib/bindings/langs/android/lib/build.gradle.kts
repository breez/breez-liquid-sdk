plugins {
    id("com.android.library")
    id("org.jetbrains.kotlin.android") version "1.8.20"
    id("maven-publish")
    kotlin("plugin.serialization") version "1.8.20"
}

repositories {
    mavenCentral()
    google()
}

android {
    compileSdk = 34

    defaultConfig {
        minSdk = 24
        consumerProguardFiles("consumer-rules.pro")
    }

    buildTypes {
        getByName("release") {
            @Suppress("UnstableApiUsage")
            isMinifyEnabled = false
            proguardFiles(file("proguard-android-optimize.txt"), file("proguard-rules.pro"))
        }
    }

    publishing {
        singleVariant("release") {
            withSourcesJar()
        }
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }
}

dependencies {
    implementation("net.java.dev.jna:jna:5.14.0@aar")
    implementation("org.jetbrains.kotlin:kotlin-stdlib-jdk7")
    implementation("androidx.appcompat:appcompat:1.6.1")
    implementation("org.jetbrains.kotlinx:kotlinx-serialization-json:1.6.3")
}

val libraryVersion: String by project

publishing {
    repositories {
        maven {
            name = "breezReposilite"
            url = uri("https://mvn.breez.technology/releases")
            credentials(PasswordCredentials::class)
            authentication {
                create<BasicAuthentication>("basic")
            }
        }
        maven {
            name = "breezGitHubPackages"
            url = uri("https://maven.pkg.github.com/breez/breez-liquid-sdk")
            credentials {
                username = System.getenv("GITHUB_ACTOR")
                password = System.getenv("GITHUB_TOKEN")
            }
        }
    }
    publications {
        create<MavenPublication>("maven") {
            groupId = "breez_sdk_liquid"
            artifactId = "bindings-android"
            version = libraryVersion

            afterEvaluate {
                from(components["release"])
            }

            pom {
                name.set("breez-liquid-sdk")
                description.set("The Breez Liquid SDK enables mobile developers to integrate Liquid swaps into their apps with a very shallow learning curve.")
                url.set("https://breez.technology")
                licenses {
                    license {
                        name.set("MIT")
                        url.set("https://github.com/breez/breez-liquid-sdk/blob/main/LICENSE")
                    }
                }
                scm {
                    connection.set("scm:git:github.com/breez/breez-liquid-sdk.git")
                    developerConnection.set("scm:git:ssh://github.com/breez/breez-liquid-sdk.git")
                    url.set("https://github.com/breez/breez-liquid-sdk")
                }
            }
        }
    }
}
