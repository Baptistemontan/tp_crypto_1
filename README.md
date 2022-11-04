# What is this ?

This is a project made to find possible cycles in hash functions.
For now 2 hash functions are available: Sha256 and Sha512.
The cycle detection method used is Floyd's algorithm.
3 partial equality functions are implemented (from fastest to slowest):
bytewise difference, the more bytes in common 2 sequences have, the closer they are.
bitwise difference, the more bits in common 2 sequences have, the closer they are.
longest common subsequence, find the biggest subsequence beetween 2 sequence, 
the bigger the sub sequence is, the closer they are.

# User manual

## How to install ?

The programm has been entirely wrote in Rust. 
Install the [Rust compiler](https://www.rust-lang.org/tools/install) on your machine in order to compile it.

## How to build ?

To build the programm simply type `cargo build --release`.

## How to use ?

Once build you can either get the newly builded binary in `target/release/tp_crypto_1` (.exe on Windows) and execute it,
or type `cargo run -- args`.

### Args

You can see them by passing the `-h` or `--help` flag to the program.
Total command: `cargo run --release -- --digest <DIGEST> --distance <DISTANCE> --threshold <THRESHOLD>  [OPTIONS]`

#### --digest

The digest argument is what hash function is used. The current choices are `sha256` and `sha512`.

#### --distance

The distance argument is what distance function is used.The possible choices are:

| distance function | description                                                                 |
|-------------------|-----------------------------------------------------------------------------|
| bit-diff          | the distance is the number of bits different beetween 2 values              |
| byte-diff         | the distance is the number of bytes different beetween 2 values             |
| common-sub-seq    | the distance is the length of the longest subsequence common in both values |

#### --threshold (-t)

The threshold argument is how low has to be the distance for 2 sequence to be considered equal.
A threshold of 0 means the 2 values has to be equals.
Each configurations as a "max" threshold, where threshold >= max will consider every pair of values to be equals:

| Hash function | distance function | max |
|---------------|-------------------|-----|
| Sha256        | bit-diff          | 256 |
| Sha256        | byte-diff         | 32  |
| Sha256        | common-sub-seq    | 32  |
| Sha512        | bit-diff          | 512 |
| Sha512        | byte-diff         | 64  |
| Sha512        | common-sub-seq    | 64  |

#### --seed (-s)

The seed is optionnal, you can either give it in utf8 (by default) or be an hex value by putting the `--hex` flag.
If the seed is not set, a 32 bytes random seed is generated.

#### --hex

Flag to mark the seed as a Hex value. ex: `-s af5e78bc --hex`.

#### exemples

`--digest sha256 --distance bit-diff -t 83 -s "Hello world"` will use the Sha256 hash function, a bitwise difference distance function with a threshold of 83 and with "Hello world" as the seed. <br/>
output: 
```
seed: 0x48656c6c6f20776f726c64
Found cycle: 
        589c73dc404cde20f15515e77fe69d9f6f701afd47b310533b2580c3438a90ac
        a0d8371448c88e03706f10c539afc98d774cc8ee4412d897bb05c04766ea92b8
Evaluated distance: 78
iterations: 61362241
Elapsed time: 97.4512275s
```
The cycle, evaluated distance and iterations are deterministic, every machine will spit out the same thing with the same arguments.