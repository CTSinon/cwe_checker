# An analysis of `cwe_checker`

## Directory Structure

```sh
.
├── Cargo.lock
├── Cargo.toml
├── LICENSE
├── Makefile
├── PcodeExtractor
│   ├── Module.manifest
│   ├── bin
│   │   ├── main
│   │   └── test
│   ├── build.gradle
│   ├── extension.properties
│   ├── ghidra_scripts      # the script to extract pcode from ghidra
│   ├── gradle
│   │   └── wrapper
│   ├── gradlew
│   └── gradlew.bat
├── README.md
└── src
    ├── caller
    │   ├── Cargo.toml
    │   └── src             # the extrypoint of this project, this part calls the ghidra script to get pcode and construct a project, then analyze it
    ├── config.json
    └── cwe_checker_lib
        ├── Cargo.toml
        └── src             # the code to analyse the pcode exported by ghidra
```
