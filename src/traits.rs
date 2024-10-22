use crate::parser::syntax::Node;
use crate::types::*;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::fmt;
use std::mem;
use std::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Not, Shl, Shr, Sub};
use std::rc::Rc;

#[cfg(any(feature = "full", feature = "parser"))]
// Vec<Box<Node>>からBox<Node>に変換するためのFromトレイト実装
impl From<Vec<Box<Node>>> for Box<Node> {
    fn from(mut vec: Vec<Box<Node>>) -> Self {
        if vec.is_empty() {
            panic!("Vec is empty");
        }

        let mut current = vec.pop().unwrap();
        while let Some(mut next) = vec.pop() {
            next.next = Rc::new(RefCell::new(Some(current)));
            current = next;
        }

        current
    }
}

#[cfg(any(feature = "full", feature = "parser"))]
// Vec<Node>からBox<Node>に変換するためのFromトレイト実装
impl From<Vec<Node>> for Box<Node> {
    fn from(vec: Vec<Node>) -> Self {
        let boxed_vec: Vec<Box<Node>> = vec.into_iter().map(Box::new).collect();
        Box::from(boxed_vec)
    }
}


