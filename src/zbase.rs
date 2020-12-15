use core::fmt;
use nom;
use nom::error::ErrorKind;
use nom::error::ParseError;
use nom::InputLength;
use nom::InputTakeAtPosition;
use nom::{branch::alt, bytes::complete::tag, combinator::value, IResult};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum ZBase {
  Z2,
  Z8,
  Z10,
  Z16,
  Z32,
  Z58,
  Z64,
}

impl Default for ZBase {
  fn default() -> Self {
    Self::Z32
  }
}

impl fmt::Display for ZBase {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Self::Z2 => write!(f, "zbase-2"),
      Self::Z8 => write!(f, "zbase-8"),
      Self::Z10 => write!(f, "zbase-10"),
      Self::Z16 => write!(f, "zbase-16"),
      Self::Z32 => write!(f, "zbase-32"),
      Self::Z58 => write!(f, "zbase-58"),
      Self::Z64 => write!(f, "zbase-64"),
    }
  }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum ZBaseError<I> {
  InvalidEncoding(I, ZBase),
  NomErr(I, ErrorKind),
}

impl<I> ZBaseError<I> {
  pub fn rest(self) -> I {
    match self {
      Self::InvalidEncoding(i, _) => i,
      Self::NomErr(i, _) => i,
    }
  }
}

impl<I> ParseError<I> for ZBaseError<I>
where
  I: InputLength,
  I: Clone,
{
  fn from_error_kind(input: I, kind: ErrorKind) -> Self {
    ZBaseError::NomErr(input, kind)
  }

  fn append(i: I, k: ErrorKind, other: Self) -> Self {
    if i.clone().input_len() < other.clone().rest().input_len() {
      ZBaseError::NomErr(i, k)
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

//impl<I: fmt::Display> fmt::Display for ZBaseError<I> {
//  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//    match self {
//      ZBaseError::InvalidEncoding(i, base) => {
//        write!(f, "Expected {} base, but got {}", base, i)
//      }
//      ZBaseError::NomErr(i, err) => {
//        write!(f, "Parse error {:?} on input {}", err, i)
//      }
//    }
//  }
//}

//
//impl<I: fmt::Debug + fmt::Display> std::error::Error for ZBaseError<I> {}

impl ZBase {
  pub fn parse_code(i: &str) -> IResult<&str, Self, ZBaseError<&str>> {
    alt((
      value(Self::Z2, tag("b")),
      value(Self::Z8, tag("o")),
      value(Self::Z10, tag("d")),
      value(Self::Z16, tag("x")),
      value(Self::Z32, tag("v")),
      value(Self::Z58, tag("I")),
      value(Self::Z64, tag("~")),
    ))(i)
  }

  /// Get the code corresponding to the base algorithm.
  pub fn code(&self) -> char {
    match self {
      Self::Z2 => 'b',
      Self::Z8 => 'o',
      Self::Z10 => 'd',
      Self::Z16 => 'x',
      Self::Z32 => 'v',
      Self::Z58 => 'I',
      Self::Z64 => '~',
    }
  }

  pub fn base_digits(&self) -> &str {
    match self {
      Self::Z2 => "01",
      Self::Z8 => "01234567",
      Self::Z10 => "0123456789",
      Self::Z16 => "0123456789abcdef",
      Self::Z32 => "ybndrfg8ejkmcpqxot1uwisza345h769",
      Self::Z58 => "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz",
      Self::Z64 => {
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_"
      }
    }
  }

  pub fn is_digit(&self, x: char) -> bool {
    self.base_digits().chars().any(|y| x == y)
  }

  pub fn encode<I: AsRef<[u8]>>(&self, input: I) -> String {
    base_x::encode(self.base_digits(), input.as_ref())
  }

  pub fn decode<'a>(
    &self,
    input: &'a str,
  ) -> IResult<&'a str, Vec<u8>, ZBaseError<&'a str>> {
    let (i, o) = input.split_at_position_complete(|x| !self.is_digit(x))?;
    match base_x::decode(self.base_digits(), o) {
      Ok(bytes) => Ok((i, bytes)),
      Err(_) => {
        Err(nom::Err::Error(ZBaseError::InvalidEncoding(i, self.clone())))
      }
    }
  }
}

pub fn parse(input: &str) -> IResult<&str, (ZBase, Vec<u8>), ZBaseError<&str>> {
  let (i, base) = ZBase::parse_code(input)?;
  let (i, bytes) = base.decode(i)?;
  Ok((i, (base, bytes)))
}

pub fn encode<T: AsRef<[u8]>>(base: ZBase, input: T) -> String {
  let input = input.as_ref();
  let mut encoded = base.encode(input.as_ref());
  encoded.insert(0, base.code());
  encoded
}

#[cfg(test)]
mod tests {
  use super::*;
  use quickcheck::{Arbitrary, Gen};
  use rand::Rng;

  impl Arbitrary for ZBase {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
      let x: u32 = g.gen();
      match x % 7 {
        0 => ZBase::Z2,
        1 => ZBase::Z8,
        2 => ZBase::Z10,
        3 => ZBase::Z16,
        4 => ZBase::Z32,
        5 => ZBase::Z58,
        6 => ZBase::Z64,
        _ => panic!("impossible"),
      }
    }
  }

  #[quickcheck]
  fn zprint_code(x: ZBase) -> bool {
    match ZBase::parse_code(&x.code().to_string()) {
      Ok((_, y)) => x == y && y.code() == x.code(),
      _ => false,
    }
  }
  #[quickcheck]
  fn zprint_string(x: ZBase, s: String) -> bool {
    match parse(&encode(x, s.clone())) {
      Ok((_, (y, s2))) => x == y && s.into_bytes() == s2,
      _ => false,
    }
  }
}
