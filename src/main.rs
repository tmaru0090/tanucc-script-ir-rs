extern crate inkwell as iw;
use iw::{
    builder::Builder,
    context::Context,
    module::Module,
    basic_block::BasicBlock,
    types::{BasicType, BasicTypeEnum, StructType},
    values::{AnyValue, AsValueRef, BasicValue, BasicValueEnum, FunctionValue, IntValue,InstructionValue},
    AddressSpace,
};

#[cfg(any(feature = "full", feature = "parser"))]
use tanucc_script_ir::parser::syntax::Parser;

#[cfg(any(feature = "full", feature = "lexer"))]
use tanucc_script_ir::lexer::tokenizer::Lexer;

#[cfg(any(feature = "full", feature = "decoder"))]
use tanucc_script_ir::decoder::ir::Decoder;
//use core::slice::SlicePattern;
use std::fs::File;
use std::io::Write;
use std::env;
use log::debug;
use tanucc_script_ir::types::R;
use tanucc_script_ir::types::IRValue;
fn with_env_var<F>(key: &str, value: &str, mut f: F)
where
    F: FnMut(),
{
    env::set_var(key, value);
    f();
    env::remove_var(key);
}

fn main() -> Result<(), String> {
    #[cfg(any(feature = "full", feature = "parser"))] {
        with_env_var("RUST_LOG", "debug", || {
            env_logger::init();
            
            let mut context = Context::create();
            let contents = String::from("100+2");
            let tokens = Lexer::from_tokenize("", contents.clone()).unwrap();
            let nodes = Parser::from_parse(&tokens, "", contents).unwrap();

            // Create builder and module in a separate scope
            let (mut builder, mut module) = {
                let builder = context.create_builder();
                let module = context.create_module("example");
                (builder, module) // Use mut here
            }; // builder and module go out of scope here

            // Move the decoder's use to a new block to avoid borrowing conflicts
            {
                let mut decoder = Decoder::new();
                // Ensure you pass the mutable context and builder here
                decoder.decode(&mut builder, &mut context, &mut module, &nodes).unwrap();
            } // decoder goes out of scope here

            debug!("{:?}", tokens);
            debug!("{:?}", nodes);
        });
        println!("Hello, world!");
    }
    Ok(())
}
