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
        5 bytes

section "function"
        2 bytes

section "export"
        5 bytes

section "code"
        6 bytes
```

# Resources

Here are a list of websites I read to learn about WebAssembly :

- [https://webassembly.org](https://webassembly.org)
- [https://webassembly.github.io/spec/core/binary/index.html](https://webassembly.github.io/spec/core/binary/index.html)
- [https://hackernoon.com/webassembly-binary-format-explained-part-2-hj1t33yp](https://hackernoon.com/webassembly-binary-format-explained-part-2-hj1t33yp)
- [https://github.com/sunfishcode/wasm-reference-manual/blob/master/WebAssembly.md](https://github.com/sunfishcode/wasm-reference-manual/blob/master/WebAssembly.md)
- [https://en.wikipedia.org/wiki/LEB128](https://en.wikipedia.org/wiki/LEB128)
