# Toy Java Virtual Machine in Rust

A toy Java Virtual Machine written in Rust.

Heavily based on [jvmrr](https://github.com/igjit/jvmrr), a toy Java Virtual Machine written in R.

# Usage 

## Compile

```
cargo build --release
```

## Run Hello World

```
./target/release/vm HelloWorld.class
```

# Java Bytecode

Compile `HelloWorld.java`:

```
javac HelloWorld.java
```

View bytecode (disassemble):

```
javap -c -verbose HelloWorld.class
```

View bytecode (hexdump):

```
hexdump -C HelloWorld.class
```
