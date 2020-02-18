# Brokkr Virtual Machine
VM for executing compiled bytecode.
The custom VM of the Valhalla Set-based Programming Language.

## Compile

In the root of this repository, in your shell, you may write:

```sh
cargo +nightly run [compiled-bytecode-file]
```

If you also have the [`valhallac`](https://github.com/valhalla-lang/valhallac) compiler cloned, you may us it to compile the `test_source.vh` file, and then run this here interpreter on the file to be unmarshalled and executed, e.g.

```sh
cargo +nightly run ../valhallac/test_source
```

### Why Nightly?

Currently, the source uses the `#![feature(const_generics)]` feature, for some function which casts an array-slice to a fixed-size type array.  It’s not really worth it, so either I wait until it becomes a proper feature, or I’ll soon remove it.

## Bytecode Reference

See the [BYTECODE.md file](BYTECODE.md) (Might not always be up-to-date).