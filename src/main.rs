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
    #[cfg(any(feature = "full", feature = "parser"))]
    {
        with_env_var("RUST_LOG", "debug", || {
            env_logger::init();

            let context = Context::create();
            let contents = String::from("1919");
            let tokens = Lexer::from_tokenize("", contents.clone()).unwrap();
            let nodes = Parser::from_parse(&tokens, "", contents).unwrap();

            // Create builder and module
            let (mut builder, mut module) = {
                let builder = context.create_builder();
                let module = context.create_module("example");
                (builder, module)
            };

            // Move the decoder's use to a new block to avoid borrowing conflicts
            {
                let mut decoder = Decoder::new();
                // Ensure you pass the mutable context and builder here
                if let Ok(IRValue::BasicValue(value)) = decoder.decode(&mut builder, &context, &mut module, &nodes){
           debug!("{:?}",value);
 /*                   let i64_type = context.i64_type();
                    let fn_type = i64_type.fn_type(&[], false);
                    let function = module.add_function("main", fn_type, None);
                    let builder = context.create_builder();
                    let basic_block = context.append_basic_block(function, "entry");
                    builder.position_at_end(basic_block);
                    builder.build_return(Some(&value));
   */             }
                // モジュールの内容を標準出力に表示
                let module_str = module.print_to_string();
                println!("{}", module_str.to_str().unwrap());
            }
            /*
            debug!("{:?}", tokens);
            debug!("{:?}", nodes);
  */
        });
        println!("Hello, world!");
    }

    Ok(())
}

/*
fn main() -> Result<(), String> {
    #[cfg(any(feature = "full", feature = "parser"))]
    {
        with_env_var("RUST_LOG", "debug", || {
            env_logger::init();

            let context = Context::create();
            let contents = String::from("1919");
            let tokens = Lexer::from_tokenize("", contents.clone()).unwrap();
            let nodes = Parser::from_parse(&tokens, "", contents).unwrap();

            // Create builder and module
            let mut builder = context.create_builder();
            let mut module = context.create_module("example");

            // Use a block to restrict the mutable borrow scope of `module`
            let value = {
                let mut decoder = Decoder::new();
                if let Ok(IRValue::BasicValue(value)) = decoder.decode(&mut builder, &context, &mut module, &nodes) {
                    value
                } else {
                    todo!();
                }
            };

            // Separate block to use the module immutably
            {
                let i64_type = context.i64_type();
                let fn_type = i64_type.fn_type(&[], false);
                let function = module.add_function("main", fn_type, None);

                let builder = context.create_builder();
                let basic_block = context.append_basic_block(function, "entry");
                builder.position_at_end(basic_block);
                builder.build_return(Some(&value));
            }

            // Output the module to stdout
            let module_str = module.print_to_string();
            println!("{}", module_str.to_str().unwrap());

            /*
            debug!("{:?}", tokens);
            debug!("{:?}", nodes);
            */
        });
        println!("Hello, world!");
    }

    Ok(())
}*/


