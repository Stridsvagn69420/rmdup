# rmdup
Simple tool for removing duplicate files

# Usage 
```sh
rmdup <file.txt>
```  
You just pass a file and it will scan the Current Working Directory for files that have the same [BLAKE3](https://github.com/BLAKE3-team/BLAKE3) hash. The file does not have to be located in the CWD too, so Absolute Paths work too.

This tool is likely to get even more functionality in the future. If you'd like to have a feature added, just open a new Issue.

## Installation
### Cargo
```sh
cargo install rmdup
```

### GitHub
Clone the repository and simply run `cargo build --release` or `cargo install --path .` or download the standalone binary for your platform on the [Release](https://github.com/Stridsvagn69420/rmdup/releases/latest) Page.