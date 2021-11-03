;; wat2wasm --enable-threads -o examples/ok2.wasm examples/ok2.wat
(module
    (memory 1 1 shared)
    (func (export "hello") (result i32)
        i32.const 42 
    )
    (func (export "world") (param $x i32) (result i32)
        local.get $x
    )
    (func (export "foobar") (param $x i32) (param $y i32) (result i32)
        (
            i32.add 
                (local.get $x)
                (local.get $y)
        )
    )
    (func (export "another_test") (param $x i32) (param $y i32) (result i32)
        (
            i32.sub 
                (local.get $x)
                (local.get $y)
        )
    )
)
