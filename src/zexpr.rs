use crate::zbase;
use crate::zbase::ZBase;
use crate::zbase::ZBaseError;
use crate::ztype;
use crate::ztype::ZType;
use crate::ztype::ZTypeError;
use std::fmt;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::multispace1;
use nom::combinator::map;
use nom::error::ErrorKind;
use nom::error::ParseError;
use nom::multi::separated_list0;
use nom::sequence::{delimited, terminated};
use nom::Err;
use nom::IResult;
use nom::InputLength;

#[derive(PartialEq, Eq, Clone)]
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
          format!("{}:{}", zbase::encode(ZBase::Z32, at), ty)
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

impl fmt::Debug for ZExpr {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Self::Atom(ty, at) => {
          format!("{}:{}", zbase::encode(ZBase::Z32, at), ty)
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
  ZBaseErr(I, ZBaseError<I>),
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

impl<I: Clone> From<ZBaseError<I>> for ZExprError<I> {
  fn from(x: ZBaseError<I>) -> Self {
    ZExprError::ZBaseErr(x.clone().rest(), x)
  }
}

impl<I: Clone> From<ZTypeError<I>> for ZExprError<I> {
  fn from(x: ZTypeError<I>) -> Self {
    ZExprError::ZTypeErr(x.clone().rest(), x)
  }
}

// <bytes>:<type>
pub fn parse_atom(i: &str) -> IResult<&str, ZExpr, ZExprError<&str>> {
  let (i, (_, at)) =
    terminated(zbase::parse, tag(":"))(i).map_err(|e| Err::convert(e))?;
  let (i, ty) = ztype::parse(i).map_err(|e| Err::convert(e))?;
  Ok((i, ZExpr::Atom(ty, at)))
}

pub fn parse(i: &str) -> IResult<&str, ZExpr, ZExprError<&str>> {
  alt((
    parse_atom,
    map(
      delimited(tag("("), separated_list0(multispace1, parse), tag(")")),
      ZExpr::Cons,
    ),
  ))(i)
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::ztype::ZType::*;
  use quickcheck::{Arbitrary, Gen, StdThreadGen};
  use rand::Rng;

  impl Arbitrary for ZExpr {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
      let gen_atom = g.gen_ratio(2, 3);
      if gen_atom {
        ZExpr::Atom(Arbitrary::arbitrary(g), Arbitrary::arbitrary(g))
      } else {
        let size = g.size();
        ZExpr::Cons(Arbitrary::arbitrary(&mut StdThreadGen::new(size / 2)))
      }
    }
  }

  #[test]
  fn zexpr_print() {
    let a = ZExpr::Atom(Bytes(None), vec![0]);
    assert_eq!(format!("{}", a), "vy:bytes");

    let c = ZExpr::Cons(vec![a.clone(), a.clone(), a.clone()]);
    assert_eq!(format!("{}", c), "(vy:bytes vy:bytes vy:bytes)");
    assert_eq!(parse(&format!("{}", c)), Ok(("", c)));
    let c = ZExpr::Cons(vec![ZExpr::Cons(vec![])]);
    assert_eq!(parse(&format!("{}", c)), Ok(("", c)));
  }

  #[quickcheck]
  fn zexpr_print_parse(x: ZExpr) -> bool {
    match parse(&format!("{}", x)) {
      Ok((_, y)) => x == y,
      _ => false,
    }
  }
}
