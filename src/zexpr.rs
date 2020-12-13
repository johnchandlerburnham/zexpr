use crate::zbase::ZBase;
use crate::zbase::*;
use crate::ztype::ZType;
use crate::ztype::ZTypeError;
//use crate::ztype::*;
use std::fmt;

use nom::error::ErrorKind;
//use nom::error::FromExternalError;
use nom::error::ParseError;
use nom::InputLength;
use nom::{
  branch::alt, bytes::complete::tag, character::complete::digit0,
  combinator::map, IResult,
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum ZExpr {
  Atom(ZType, Vec<u8>),
  Cons(Vec<ZExpr>),
}

impl fmt::Display for ZExpr {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Self::Atom(ty, at) => {
          format!("{}:{}", encode(ZBase::Z32, at), ty)
        }
        Self::Cons(xs) => {
          format!(
            "({})",
            xs.iter()
              .map(|x| format!("{}", x))
              .collect::<Vec<String>>()
              .join(" ")
          )
        }
      }
    )
  }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum ZExprError<I> {
  ZTypeErr(I, ZTypeError<I>),
  ZBaseErr(I, ZBaseError),
  NomErr(I, ErrorKind),
}

impl<I> ZExprError<I> {
  pub fn rest(self) -> I {
    match self {
      Self::ZTypeErr(i, _) => i,
      Self::ZBaseErr(i, _) => i,
      Self::NomErr(i, _) => i,
    }
  }
}

impl<I> ParseError<I> for ZExprError<I>
where
  I: InputLength,
  I: Clone,
{
  fn from_error_kind(input: I, kind: ErrorKind) -> Self {
    ZExprError::NomErr(input, kind)
  }

  fn append(i: I, k: ErrorKind, other: Self) -> Self {
    if i.clone().input_len() < other.clone().rest().input_len() {
      ZExprError::NomErr(i, k)
    } else {
      other
    }
  }

  fn or(self, other: Self) -> Self {
    if self.clone().rest().input_len() < other.clone().rest().input_len() {
      self
    } else {
      other
    }
  }
}

//pub fn parse_atom(input: &str) -> IResult<&str, ZType, ZExprError<&str>> {
//
//
//}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::ztype::ZType::*;

  #[test]
  fn zexpr_print() {
    let a = ZExpr::Atom(Bytes(None), vec![0]);
    assert_eq!(format!("{}", a), "vy:bytes");

    let c = ZExpr::Cons(vec![a.clone(), a.clone(), a.clone()]);
    assert_eq!(format!("{}", c), "(vy:bytes vy:bytes vy:bytes)");
  }
}
