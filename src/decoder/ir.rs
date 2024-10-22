use crate::parser::syntax::{Parser,Node};
use crate::types::R;
use crate::types::IRValue;
extern crate inkwell as iw;
use iw::{
    builder::Builder,
    context::Context,
    module::Module,
    types::{BasicType, BasicTypeEnum, StructType},
    values::{AnyValue, AsValueRef, BasicValue, BasicValueEnum, FunctionValue, IntValue},
    AddressSpace,
};

pub struct Decoder{
}
impl Decoder{
    pub fn new()->Self{
        Decoder{
        }
    }
    pub fn decode<'ctx>(&mut self,builder:&'ctx mut Builder,context:&'ctx mut Context,module:&'ctx mut Module,node:&Node)->R<IRValue,String>{
        Err("".to_string())
    }
}

