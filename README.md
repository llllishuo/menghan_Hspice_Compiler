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
```shell
./HspiceComplier [data file] [output paht]
```
**OR**
```shell
./HspiceComplier [-h/--help]
```

### Environmental construction
