;; wat2wasm -o examples/ok1.wasm examples/ok1.wat
(module
    (memory 1 1 shared)
    (func (export "f") (result i32)
        i32.const 42 
    )
    (func (export "a") (param $x i32) (result i32)
        local.get $x
    )
    (func (export "s") (param $x i32) (param $y i32) (result i32)
        (
            i32.add 
                (local.get $x)
                (local.get $y)
        )
    )
)
