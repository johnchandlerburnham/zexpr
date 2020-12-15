use std::fmt;

use nom::error::ErrorKind;
use nom::error::FromExternalError;
use nom::error::ParseError;
use nom::sequence::preceded;
use nom::InputLength;
use nom::{
  branch::alt, bytes::complete::tag, character::complete::digit0,
  combinator::map, IResult,
};
use std::num::ParseIntError;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum ZType {
  Bytes(Option<u64>),
  Symbol(Option<u64>),
  Nat(Option<u64>),
  Int(Option<u64>),
  Float(Option<u64>),
  Text(Option<u64>),
  Char(Option<u64>),
  Hash(Option<u64>),
}

impl fmt::Display for ZType {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let index_str = |index: Option<u64>| -> String {
      match index {
        Some(a) => (a * 8).to_string(),
        None => String::from(""),
      }
    };
    write!(
      f,
      "{}",
      match self {
        Self::Bytes(x) => format!("{}{}", "bytes", index_str(*x)),
        Self::Symbol(x) => format!("{}{}", "symbol", index_str(*x)),
        Self::Nat(x) => format!("{}{}", "nat", index_str(*x)),
        Self::Int(x) => format!("{}{}", "int", index_str(*x)),
        Self::Float(x) => format!("{}{}", "float", index_str(*x)),
        Self::Text(x) => format!("{}{}", "text", index_str(*x)),
        Self::Char(x) => format!("{}{}", "char", index_str(*x)),
        Self::Hash(x) => format!("{}{}", "hash", index_str(*x)),
      }
    )
  }
}

impl ZType {
  pub fn serialize(&self) -> &[u8] {
    match self {
      Self::Bytes(_) => &[0x00],
      Self::Symbol(_) => &[0x01],
      Self::Nat(_) => &[0x02],
      Self::Int(_) => &[0x03],
      Self::Float(_) => &[0x04],
      Self::Text(_) => &[0x05],
      Self::Char(_) => &[0x06],
      Self::Hash(_) => &[0x07],
    }
  }
  pub fn deserialize(i: &[u8], len: Option<u64>) -> Option<Self> {
    match i {
      &[0x00] => Some(Self::Bytes(len)),
      &[0x01] => Some(Self::Symbol(len)),
      &[0x02] => Some(Self::Nat(len)),
      &[0x03] => Some(Self::Int(len)),
      &[0x04] => Some(Self::Float(len)),
      &[0x05] => Some(Self::Text(len)),
      &[0x06] => Some(Self::Char(len)),
      &[0x07] => Some(Self::Hash(len)),
      _ => None,
    }
  }
  pub fn is_some_len(&self) -> bool {
    match self {
      Self::Bytes(Some(_)) => true,
      Self::Symbol(Some(_)) => true,
      Self::Nat(Some(_)) => true,
      Self::Int(Some(_)) => true,
      Self::Float(Some(_)) => true,
      Self::Text(Some(_)) => true,
      Self::Char(Some(_)) => true,
      Self::Hash(Some(_)) => true,
      _ => false,
    }
  }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum ZTypeError<I> {
  UnalignedTypeIndex(I, u64),
  InvalidU64TypeIndex(I, ParseIntError),
  NomErr(I, ErrorKind),
}

impl<I> ZTypeError<I> {
  pub fn rest(self) -> I {
    match self {
      Self::UnalignedTypeIndex(i, _) => i,
      Self::InvalidU64TypeIndex(i, _) => i,
      Self::NomErr(i, _) => i,
    }
  }
}

impl<I> ParseError<I> for ZTypeError<I>
where
  I: InputLength,
  I: Clone,
{
  fn from_error_kind(input: I, kind: ErrorKind) -> Self {
    ZTypeError::NomErr(input, kind)
  }

  fn append(i: I, k: ErrorKind, other: Self) -> Self {
    if i.clone().input_len() < other.clone().rest().input_len() {
      ZTypeError::NomErr(i, k)
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

impl<I> FromExternalError<I, ZTypeError<I>> for ZTypeError<I> {
  fn from_external_error(_: I, _: ErrorKind, e: ZTypeError<I>) -> Self {
    e
  }
}

pub fn parse_index(i: &str) -> IResult<&str, Option<u64>, ZTypeError<&str>> {
  let (i, o) = digit0(i)?;
  if o.len() == 0 {
    Ok((i, None))
  } else {
    match o.parse::<u64>() {
      Ok(x) => {
        if x % 8 == 0 {
          Ok((i, Some(x / 8)))
        } else {
          Err(nom::Err::Error(ZTypeError::UnalignedTypeIndex(i, x)))
        }
      }
      Err(e) => Err(nom::Err::Error(ZTypeError::InvalidU64TypeIndex(i, e))),
    }
  }
}

pub fn parse(input: &str) -> IResult<&str, ZType, ZTypeError<&str>> {
  alt((
    map(preceded(tag("bytes"), parse_index), ZType::Bytes),
    map(preceded(tag("symbol"), parse_index), ZType::Symbol),
    map(preceded(tag("nat"), parse_index), ZType::Nat),
    map(preceded(tag("int"), parse_index), ZType::Int),
    map(preceded(tag("float"), parse_index), ZType::Float),
    map(preceded(tag("text"), parse_index), ZType::Text),
    map(preceded(tag("char"), parse_index), ZType::Char),
    map(preceded(tag("hash"), parse_index), ZType::Hash),
  ))(input)
}

#[cfg(test)]
mod tests {
  use super::*;
  use nom::Err::Error;
  use quickcheck::{Arbitrary, Gen};
  use rand::Rng;

  impl Arbitrary for ZType {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
      let x: u32 = g.gen();
      match x % 8 {
        0 => ZType::Bytes(Arbitrary::arbitrary(g)),
        1 => ZType::Symbol(Arbitrary::arbitrary(g)),
        2 => ZType::Nat(Arbitrary::arbitrary(g)),
        3 => ZType::Int(Arbitrary::arbitrary(g)),
        4 => ZType::Float(Arbitrary::arbitrary(g)),
        5 => ZType::Text(Arbitrary::arbitrary(g)),
        6 => ZType::Char(Arbitrary::arbitrary(g)),
        7 => ZType::Hash(Arbitrary::arbitrary(g)),
        _ => panic!("impossible"),
      }
    }
  }

  #[quickcheck]
  fn ztype_string(x: ZType) -> bool {
    match parse(&format!("{}", x)) {
      Ok((_, y)) => x == y,
      _ => false,
    }
  }

  #[test]
  fn test_parse() {
    //assert_eq!(parse_ztype("bytes"), Ok(("", ZType::Bytes(None))));
    assert_eq!(parse_index("8"), Ok(("", Some(1))));

    assert_eq!(
      parse_index("9"),
      Err(nom::Err::Error(ZTypeError::UnalignedTypeIndex("", 9)))
    );
    assert_eq!(format!("{}", ZType::Bytes(Some(1))), String::from("bytes8"));

    let mut parser =
      map(preceded(tag("bytes"), parse_index), |x| ZType::Bytes(x));

    assert_eq!(
      parser("bytes9"),
      Err(Error(ZTypeError::UnalignedTypeIndex("", 9)))
    );
    assert_eq!(
      parse("bytes9"),
      Err(Error(ZTypeError::UnalignedTypeIndex("", 9)))
    );
  }
}
