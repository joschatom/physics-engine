[![Publish Release](https://github.com/joschatom/physics-engine/actions/workflows/publish_release.yml/badge.svg?branch=stable)](https://github.com/joschatom/physics-engine/actions/workflows/publish_release.yml)

# Physics Engine

## About
A Simple Physics Engine I made using **`macroquad`** and **`specs`**

## Installing
If there's a release available download the latest one otherwise refer to [this section](#compiling-from-source)

## Compiling from Source

### Cloning the Repo
You must have **`git`** installed, then run the following command to clone the repository and cd into the directory

```sh
git clone https://github.com/joschatom/physics-engine.git
cd physics-engine
git switch -b stable # Remove this if you want to compile the unstable codebase.
```

### Installing Rust
For Instructions on how to install Rust visit the [**Rust** website](https://www.rust-lang.org)

### Building and Running
First, make sure you have the repository cloned(refer to [this section](#cloning-the-repo)).
Next, make sure you are in the project directory.

And then build using this command: ```sh
cargo build --bin physics-engine```

And if you want to run it use this command: ```sh
cargo run --bin physics-engine```

### Compiling in **Release** mode
To compile in release mode add the `--release` flag to the command shown in [Building and Running](#builing-and-running)

## Note
This is a hobby side-project so it might have bugs, but if you find them opening an issue, would help me fix it!

