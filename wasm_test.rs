// Adapted from https://github.com/bytecodealliance/wasmtime/blob/main/docs/WASI-tutorial.md
pub mod wasm_test {

    pub fn wasm_test(pid_self: i64) {

        let wat: &str = r#"
            (module
                (import "lunatic::message" "create_data" (func $create_data (param i64 i64)))
                (import "lunatic::message" "write_data" (func $write_data (param i32 i32) (result i32)))
                (import "lunatic::message" "send" (func $send (param i64) (result i32)))
    
                ;; Const data.
                (memory 1)
                (export "memory" (memory 0))
                (data (i32.const 8) "hello world")
    
                (func (export "hello") (param $pid_dest i64)

                    ;; Create buffer for message. Unspecified Id, capacity of 11.
                    (call $create_data (i64.const 0) (i64.const 11))
    
                    ;; Write data to the message buffer.
                    (call $write_data
                        (i32.const 8) ;; Pointer.
                        (i32.const 11) ;; Size of buffer.
                    )
    
                    ;; Send message buffer, parameterized for now.
                    (call $send (local.get $pid_dest))
                    return
                )
            )
        "#;

        lunatic::WasmModule::new(wat.as_bytes())
            .unwrap()
            .spawn_link::<i32, lunatic::serializer::Bincode>(
                "hello", &[lunatic::Param::I64(pid_self.try_into().unwrap())], lunatic::Tag::new()
            )
            .unwrap();
    }

    #[lunatic::test]
    fn wasm_spawn_test(m: lunatic::Mailbox<Vec<u64>>) {
        wasm_test(m.this().id() as i64);
        assert_eq!(vec![1 as u64, 2 as u64, 3 as u64], m.receive());
    }
}
