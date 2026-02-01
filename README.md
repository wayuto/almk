# Almk
A C/C++ build tool like cargo

## *Installation*
### *Dependencies*

| Dependency            | Version    |
| --------------------- | ---------- |
| Rust Nightly && Cargo | >= 1.93.0  |
| clap                  | >= 4.5.54  |
| serde                 | >= 1.0.22  |
| serde_json            | >= 1.0.149 |
| sha2                  | >= 0.10.9  |
| toml =                | >= 0.9.11  |
| walkdir               | >= 2.5.0   |

### *Install from source*
```bash
$ cargo install --path .
```

## Quick Start
### Create a new project
```bash
# Creating a C project
$ almk new hello # or add "--language C"
'hello' has been created successfully.

# Creating a C++ project
$ almk new hello --language C++
'hello' has been created successfully.
```
### Build the project
```bash
$ almk build
clang -g -O0 -DDEBUG  -c ./src/main.c -o target/objects/main.o -MMD
clang target/objects/main.o -o target/hello
```

### Run the project
```bash
$ almk run # almk supports incremental compilation, so if you have ran "almk build", if will run "target/hello" directly.
Hello world!
```

### Clean up
```bash
$ almk clean
Removed object files in 'target' directory.
```

### Add a dependency
```bash
$ almk add util -u https://www.website.com/util.zip # or almk add util -u https://www.website.com/util.git -g
Added dependency 'util'
```

### Remove a dependency
```bash
$ almk rm util
```

(Though this is a quick start, it's all usage of `almk` so far...)

## *Features*
### Automatically find and compile all C/C++ files in `src/`
You don't need to add each file to a configuration file like cmake, or use the unreadable syntax for human like make (I hate it so I created `almk`).
### Incremental compilation
As mentioned above, `almk` supports `incremental compilation`, it'll only compile modified sources. As an example, it'll run the output file directly without compiling again (though it's an easy feature, I really like because I put a lot thought into it). It depends on `target/deps.json`
### Managing projects by `Alumake.toml`
For `almk`, you can manage your project by `Alumake.toml` like manage your rust project by `Cargo.toml`. It's really easier than `CMakelists.txt` or `Makefile` (At least for me). However, as for now, it's too easy to manage a large project, it's better suited for managing small personal project (I'm making efforts to improve it).

## *Configuration*
For `almk`, you'll manage all of your project by `Alumake.toml`. In this chapter, I'll introduce it clearly to you.  
A defalut `Alumake.toml` just like following
```toml
[package]
name = "your project name"
version = "0.1.0"
author = ""
license = "No License"
language = "C"

[build]
debug = true
linker = "clang"
cc = "clang"
```
I'll introduce each field in detail.
### Package
This field deined all metadata for your project.  
Some feilds are opitonal but reommended, they can help you avoid some unnecessary trouble.
| Field    | Type   | Optinality |
| -------- | ------ | ---------- |
| name     | String | required   |
| version  | String | optional   |
| author   | String | optional   |
| license  | String | optional   |
| language | String | optional   |
### Build
As the name said, all of the compilation options will be defined in this field.
| Field    | Type        | Optionality | Description                               |
| -------- | ----------- | ----------- | ----------------------------------------- |
| debug    | bool        | required    | optimization for compilation              |
| linker   | String      | required    | linking `target/objects/*.o` to execuable |
| cc       | String      | optional    | required if you'll compile c file         |
| cxx      | String      | optional    | required if you'll compile c++ file       |
| cflags   | String      | optional    | compilation parameters for C compiler     |
| cxxflags | String      | optional    | compilation parameters for C++ compiler   |
| lnflags  | String      | optional    | compilation parameters for linker         |
| includes | Vec<String> | optional    | add `-I./path/to/include` when compiling  |
### Dependenciess
- For `.zip` file:
```toml
[dependencies.dep]
url = "https://www.website.com/dep.zip"
git = false
```
- For `git` repo:
```toml
[dependencies.dep]
url = "https://www.website.com/dep.git"
git = true
tag = "v1.0"
```
- For `local` file:
```toml
[dependencies.dep]
local = "/path/to/dep"
git = false
```