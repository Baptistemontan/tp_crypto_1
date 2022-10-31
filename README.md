# What is this ?

This is a project made to find possible cycles in hash functions.
For now 2 hash functions are used: Sha256 and Sha512.
The cycle detection method used is Floyd's algorithm.
3 partial equality functions are implemented (from fastest to slowest):
bytewise difference, the more bytes in common 2 sequences have, the closer they are.
bitwise difference, the more bits in common 2 sequences have, the closer they are.
longest common subsequence, find the biggest subsequence beetween 2 sequence, 
the bigger the sub sequence is, the closer they are.

# User manual

## How to install ?

The programm has been entirely wrote in Rust. 
Install the Rust compiler on your machine in order to compile it.

## How to build ?

To build the programm simply type `cargo build --release`.

### Why `--release` ?
The `release` flag is optionnal, but is important, it enable compiler optimizations (equivalent to -02 in C/C++).
On my machine I see a ~35x speed increase.

#### Bench 1:

With digest = Sha256, distance function = bitwise, threshold = 86 and seed = 0x00, it takes 3.198.100 iterations.
without the release flag: ~189s (still faster than python tho).
with the release flag: < 5s.

#### Bench 2:

With digest = Sha256, distance = bytewise, threshold = 


## How to use ?

Once build you can either get the newly builded binary in `target/release/tp_crypto_1` (.exe on Windows) and execute it,
or type `cargo run -- args`.

### Args

You can see them by passing the `-h` or `--help` flag to the program.
The digest argument is what hash function is used.
the distance argument is what distance function is used.
the threshold argument is how low has to be the distance for 2 sequence to be considered equal.
the seed is optionnal, you can either give it in utf8 (by default) or be an hex value by putting the `--hex` flag.
If the seed is not set, a 32 bytes random seed is generated 

ex: `--digest sha256 --distance bit-diff -t 86 -s 00 --hex` will recreate the first benchmark.
