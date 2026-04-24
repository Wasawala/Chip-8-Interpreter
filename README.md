
# Chip-8 Interpreter

A simple command line Chip-8 interpreter made in rust.


## Compilation 

You will need to have cargo installed.

To compile, type:

```bash
cargo build
```

## Usage

```bash
cargo run <FILE> [width] [height] [fullscreen]
```

So launching it with file "test.ch8", size 850x1000, and without fullscreen should look like this:

```bash
cargo run "test.ch8" 850 1000 
```

For more help run:

```bash
 cargo run -- -h
```


