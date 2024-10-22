use crate::parser::syntax::{Parser,Node};
use crate::types::{R,IRValue,Expr,Statement,Operator,DataType,NodeValue};
extern crate inkwell as iw;
use iw::{
    builder::Builder,
    context::Context,
    module::Module,
    types::{BasicType, BasicTypeEnum, StructType},
    values::{AnyValue, AsValueRef, BasicValue, BasicValueEnum, FunctionValue, IntValue},
    AddressSpace,
};

pub struct Decoder;
impl Decoder {
    pub fn new()->Self{Decoder{}}
    pub fn decode<'ctx>(&mut self, builder: &'ctx mut Builder, context: &'ctx Context, module: &'ctx mut Module, node: &Node) -> R<IRValue<'ctx>, String> {
        match &node.value {
            NodeValue::Expr(ref expr) => {
                match *expr {
                    Expr::DataType(ref datatype) => {
                        match *datatype {
                            DataType::Int(value) => {
                                let int_value = context.i64_type().const_int(value as u64, false);
                                Ok(IRValue::BasicValue(int_value.as_basic_value_enum()))
                            },
                            DataType::Float(value) => {
                                let float_value = context.f64_type().const_float(value);
                                Ok(IRValue::BasicValue(float_value.as_basic_value_enum()))
                            },
                            _ => todo!(),
                        }
                    },
                        /*
                    Expr::Operator(ref op) => {
                        match *op {
                            Operator::Add(ref left, ref right) => {
                                // まずleftを計算し、借用期間を終了させる
                                let left_ir_value = self.decode(builder, context, module, &*left)?;
                                let left_value = left_ir_value.as_basic_value();

                                // rightの計算を別のスコープで行う
                                let right_ir_value = {
                                    self.decode(builder, context, module, &*right)?
                                };
                                let right_value = right_ir_value.as_basic_value();
                                
                                // 加算処理にmap_errを使用
                                let sum = builder.build_int_add(left_value.into_int_value(), right_value.into_int_value(), "tmpadd")
                                    .map_err(|e| e.to_string())?;
                                
                                Ok(IRValue::BasicValue(sum.as_basic_value_enum()))
                            },
                            _ => todo!(),
                        }
                    },*/
                    _ => todo!(),
                }
            },
            _ => todo!(),
        }
    }
}
