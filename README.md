![Not Working Yet](./img/badges/notworkingyet.svg)

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

cargo run examples/ok2.wasm

```text
Wasm magic 6d736100   
        version 1     

section "type"        
        0: fn() -> i32
        1: fn(i32) -> i32
        2: fn(i32, i32) -> i32

section "function"
        range=[28-32]
        04 00 01 02 02
        found 4 functions

section "memory"
        range=[35-38]
        01 03 01 01

section "export"
        0: hello type=Function index=0
        1: world type=Function index=1
        2: foobar type=Function index=2
        3: another_test type=Function index=3

section "code"
        0:      0 local
                41 2a           i32.const 42
                0b              end

        1:      0 local
                20 00           local.get 0
                0b              end

        2:      0 local
                20 00           local.get 0
                20 01           local.get 1
                6a              i32.add
                0b              end

        3:      0 local
                20 00           local.get 0
                20 01           local.get 1
                6b              i32.sub
                0b              end
```

# Resources

Here are a list of websites I read to learn about WebAssembly :


- [https://webassembly.org](https://webassembly.org)
- [https://webassembly.github.io/spec/core/binary/index.html](https://webassembly.github.io/spec/core/binary/index.html)
- [https://github.com/sunfishcode/wasm-reference-manual/blob/master/WebAssembly.md](https://github.com/sunfishcode/wasm-reference-manual/blob/master/WebAssembly.md)
- [https://en.wikipedia.org/wiki/LEB128](https://en.wikipedia.org/wiki/LEB128)
