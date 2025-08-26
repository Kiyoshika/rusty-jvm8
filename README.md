# Rusty JVM8
This is an educational project to implement a somewhat Java 8 compliant JVM in Rust.

Java 8 specification can be found [here](https://docs.oracle.com/javase/specs/jvms/se8/html/index.html).

## Major Milestones Roadmap
### 1. Parse Class Files [IN PROGRESS]
- [X] Read magic number
- [ ] Read minor version
- [ ] Read major version
- [ ] Read constant pool
- [ ] Read access flags
- [ ] Read this class
- [ ] Read super class
- [ ] Read interfaces
- [ ] Read fields
- [ ] Read methods
- [ ] Read attributes
### 2. Add support for all JVM instructions [NOT STARTED]
### 3. Implement class loader [NOT STARTED]
### 4. Create JVM CLI and runtime (run java code) [NOT STARTED]

## Adding/Compiling Java Tests
If you are adding new Java code and compiling new class files for tests, you will need to install the Java 8 SDK.

There's a `.sdkmanrc` file that specifies the Java 8 SDK to use if compiling any java code used for generating class files for tests (`tests/java/...`)

1. `brew install sdkman` (if not already)
2. run `sdk env` from root directly to install and use the specified Java 8 SDK