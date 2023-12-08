# Hspice Comepler

+ bin 
+ common : Tool class
    - offic
+ data : sample data
+ hspice : Main business methods
    - analysis
    - circuit
    - device
    - source
    - spice

### build && run

```shell
cargo build
```

```shell
cargo run -- [data file] [output path]
```

***OR***

```shell
cargo run -- [-h/--help]
```

### If binary files are needed
- install cross
```shell
cargo install cross
```
- Build and Execute binary files
```shell
cross build --release --target x86_64-pc-windows-gnu
cross build --release --target x86_64-apple-darwin
cross build --release --target x86_64-unknown-linux-gnu
```
**This will build binary files on Windows, Mac, and Linux respectively. After the construction is completed, you will be working on target/x86_ 64 pc windows gnu/release, target/x86_ 64 apple darwin/release and target/x86_ Three binary files were found in the 64 unknown Linux gnu/release directory.**
```shell
./HspiceComplier [data file] [output paht]
```
**OR**
```shell
./HspiceComplier [-h/--help]
```

### Environmental construction
