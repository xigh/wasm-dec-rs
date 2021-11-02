# Introduction

This program *wasm-dec-rs* is a simple [wasm](https://webassembly.org/) binary decoder. My first official [rust](https://www.rust-lang.org/) program in fact.

I had to write this in order to learn how it works and how we can "tweak" / "optimize" rust wasm32-unknown-unknown generated binaries.

I'll describe how as soon as possible in another github repository.

# Usage

## example 1: (module) only

cat examples/ok0.wat

```wat
;; wat2wasm -o examples/ok0.wasm examples/ok0.wat
(module)
```

cargo run examples/ok0.wasm

```text
Wasm magic 6d736100
        version 1
```

## example 2: module with exported function (TODO: fill the blanks)

cat examples/ok1.wat

```wat
;; wat2wasm -o examples/ok1.wasm examples/ok1.wat
(module
    (func (export "f") (result i32)
        i32.const 42 
    )
)
```

cargo run examples/ok1.wasm

```text
Wasm magic 6d736100
        version 1

section "type"
        range=[10-25]
        03 60 00 01 7f 60 01 7f 01 7f 60 02 7f 7f 01 7f
        0: fn() -> i32
        1: fn(i32) -> i32
        2: fn(i32, i32) -> i32

section "function"
        range=[28-31]
        03 00 01 02

section "memory"
        range=[34-37]
        01 03 01 01

section "export"
        range=[40-52]
        03 01 66 00 00 01 61 00 01 01 73 00 02

section "code"
        range=[55-73]
        03 04 00 41 2a 0b 04 00 20 00 0b 07 00 20 00 20 01 6a 0b 
```

# Resources

Here are a list of websites I read to learn about WebAssembly :


- [https://webassembly.org](https://webassembly.org)
- [https://webassembly.github.io/spec/core/binary/index.html](https://webassembly.github.io/spec/core/binary/index.html)
- [https://github.com/sunfishcode/wasm-reference-manual/blob/master/WebAssembly.md](https://github.com/sunfishcode/wasm-reference-manual/blob/master/WebAssembly.md)
- [https://en.wikipedia.org/wiki/LEB128](https://en.wikipedia.org/wiki/LEB128)
