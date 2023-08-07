use inkwell::context::Context;
use inkwell::execution_engine::JitFunction;
use inkwell::module::Module;
use inkwell::memory_buffer::MemoryBuffer;
use std::error::Error;
use std::fs;
use std::process::{Command, Stdio};
use std::io::{self, Write};
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    // Example Rust code as a string (loaded from the database)
    let rust_code = r#"
        #[no_mangle]
        pub extern "C" fn add(a: i8, b: i8) -> i8 {
            a + b
        }
    "#;

    // Compile the Rust code to LLVM IR using rustc
    let mut rustc_process = Command::new("rustc")
        .args(&[
            "-",
            "--emit=llvm-ir",
            "--crate-type=lib",
            "-C", "opt-level=3",
        ])
        .stdin(Stdio::piped())
        .stdin(Stdio::piped())
        .spawn()?;

    // Instead of from physical file. I'm compiling from memory.
    if let Some(stdin) = rustc_process.stdin.as_mut() {
        stdin.write_all(rust_code.as_bytes())?;
    }
    let output = rustc_process.wait_with_output()?;

    if output.status.success() {
        // Create a context and module
        let context = Context::create();
        let module = context.create_module("main");
        println!("Rust code compiled successfully to LLVM IR");
        let rust_code_ir = "rust_out.ll"; // Replace with the path to your generated LLVM IR file

        // Load IR from file into a MemoryBuffer
        let ir_contents = std::fs::read(rust_code_ir)?;
        // fs::remove_file(rust_code_ir)?;
        let memory_buffer = MemoryBuffer::create_from_memory_range(&ir_contents, "temp.ll");
        
        // Create a module from the MemoryBuffer containing LLVM IR
        let rust_code_module = context.create_module_from_ir(memory_buffer)?;
        module.link_in_module(rust_code_module)?;

        // Create an execution engine
        let jit = module.create_jit_execution_engine(Default::default())?;

        // Get a pointer to the compiled function
        let compiled_fn: JitFunction<unsafe extern "C" fn(i16, i16) -> i16> = unsafe { jit.get_function("add")? };

        // Call and print the result
        for num in 1..=100 {
            let x = 10;
            let y = 20;
            let result = unsafe { compiled_fn.call(x * num, y / num) };
            println!("Result: {}", result);    
        }
    } else {
        eprintln!("Error compiling Rust code: {:?}", output.stderr);
    }
    Ok(())
}