# Physics Engine

## About
A Simple Physics Engine I made using **`macroquad`** and **`specs`**

## Installing
If there's an release available download the latest one otherwise refer to [this section](#compiling-from-source)

## Compiling from Source

### Cloning the Repo
You must have **`git`** installed, then run following command to clone the repository and cd into the directory

```sh
git clone https://github.com/joschatom/physics-engine.git
cd physics-engine
```

### Insalling Rust
For Instructions on how to install rust visit the [**Rust** website](https://www.rust-lang.org)

### Building and Running
First make sure you have the repository cloned(refer to [this section](#cloning-the-repo)).
Next make sure you in the porject directory.

And then build using this command: ```sh
cargo build --bin physics-engine```

And if you want to run it use this command: ```sh
cargo run --bin physics-engine```

### Compiling in **Release** mode
To compile in release mode add the `--release` flag to the commad show in [Building and Running](#builing-and-running)

## Note
This is a hobby side-project so it might have bugs, but if you find them opening a issue would help me fix it!
