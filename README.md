
# Chip-8 Interpreter

A simple command line Chip-8 interpreter made in rust.


## Compilation 

You will need to have cargo installed.

To compile, type:

```bash
cargo build --release
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

## Controls

CHIP-8 Keypad        Keyboard
+-+-+-+-+            +-+-+-+-+
|1|2|3|C|            |1|2|3|4|
+-+-+-+-+            +-+-+-+-+
|4|5|6|D|    =>      |Q|W|E|R|
+-+-+-+-+            +-+-+-+-+
|7|8|9|E|            |A|S|D|F|
+-+-+-+-+            +-+-+-+-+
|A|0|B|F|            |Z|X|C|V|
+-+-+-+-+            +-+-+-+-+

You can also quit the program using the ESCAPE key.


