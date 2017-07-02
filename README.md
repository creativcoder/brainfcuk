# brainfcuk

An optimizing brainfuck interpreter written in Rust

This implementation preprocess the source code with following optimizations:

1) Performs run length encoding (RLE) of the source text. This obviously shows best results when source has more repeated characters, which are folded into constant arithmetic operations.

2) Eliminates [-] which basically turns into an expression which sets the current cell to zero, replacing it with `|` symbol, which is handled by the interpreter appropriately.

3) Precomputes brace indices on the RLE source for close to constant time loop jumps.

The interpreter builds up its own custom IR and evaluates that IR to compute the end results from the cell.

TODO:
* Add LLVM backend to compile to native code.
* Optimize code to minimize string allocations.
* Better Error messages
