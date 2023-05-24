# arcilator-bind

You're intended to use this with the `arcilator` binary built with CIRCT,
which can be used to lower the HW dialect into LLVM IR.
See [circt/tools/arcilator](https://github.com/llvm/circt/tree/main/tools/arcilator)
and [circt/arc-tests](https://github.com/circt/arc-tests) for more context.

Currently, it seems like `arcilator` generates a JSON description of the 
simulator state. In the CIRCT tree, `arcilator-header-cpp.py` uses this 
to emit a C++ header with an interface to the simulator. 
I suppose we can try doing this for Rust as well. 

For instance: 

```
# Lower to the HW dialect
$ firtool --ir-hw MyFile.fir > MyFile.mlir.hw

# Lower to the ARC dialect
$ arcilator --state-file=MyFile.state.json MyFile.mlir.hw > MyFile.bc

# Lower to native code
$ opt -O3 --strip-debug -S MyFile.bc | llc -O3 --filetype=obj -o MyFile.o

# Generate bindings
...

...
```
