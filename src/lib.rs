#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;
#[cfg(test)]
extern crate rand;

extern crate nom;

pub mod zbase;
pub mod ztype;

use nom::branch::alt;
use nom::bytes::complete::{tag, take};
use nom::character::complete::multispace1;
use nom::combinator::map;
use nom::error::ErrorKind;
use nom::error::ParseError;
use nom::multi::{count, separated_list0};
use nom::sequence::{delimited, terminated};
use nom::Err;
use nom::IResult;
use nom::InputLength;

use std::fmt;
use zbase::ZBase;
use zbase::ZBaseError;
use ztype::ZType;
use ztype::ZTypeError;

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

//impl Iterator for ZExpr {
//  fn next(&mut self) -> Option<(ZType, Vec<u8>)> {
//    match self {
//      Self::Atom(ty, at) => Some((ty, at)),
//      Self::Cons(xs) => xs.next().next(),
//    }
//  }
//}

impl ZExpr {
  pub fn serialize(&self) -> Vec<u8> {
    match self {
      Self::Atom(typ, dat) => {
        let typ_len: u8 = typ.serialize().len() as u8;
        let dat_len: u64 = dat.len() as u64;
        let dat_len_len: u8 = number_of_bytes(dat_len);

        //println!("ser dat_len {}", dat_len);
        let dat_len: Vec<u8> = dat_len.to_le_bytes()[0..(dat_len_len as usize)]
          .to_vec()
          .into_iter()
          .rev()
          .collect();
        //println!("ser dat_len {:?}", dat_len);

        //println!("ser typ_len {}", typ_len);
        //println!("ser dat_len_len {}", dat_len_len);
        let size_byte: u8 = if typ.is_some_len() {
          (0b0111_1111) & (1 << 6 | (typ_len - 1) << 3) | (dat_len_len - 1)
        } else {
          (0b0011_1111) & ((typ_len - 1) << 3) | (dat_len_len - 1)
        };

        let mut ret = vec![];

        ret.extend(vec![size_byte]);
        ret.extend(typ.serialize());
        ret.extend(dat_len);
        ret.extend(dat);
        //println!("ser ret {:?}", ret);
        ret
      }
      Self::Cons(xs) => {
        let xs_len: u64 = xs.len() as u64;
        let xs_len_len: u8 = number_of_bytes(xs_len);
        let xs_len: Vec<u8> = xs_len.to_le_bytes()[0..(xs_len_len as usize)]
          .to_vec()
          .into_iter()
          .rev()
          .collect();
        let size_byte: u8 = 0b1000_0111 & (0b1000_0000 | (xs_len_len - 1));
        let mut ret: Vec<u8> = vec![];
        ret.extend(vec![size_byte]);
        ret.extend(xs_len);
        ret.extend(xs.iter().fold(vec![], |mut acc, x| {
          acc.extend(ZExpr::serialize(x));
          acc
        }));
        ret
      }
    }
  }

  pub fn deserialize(
    i: &[u8],
  ) -> IResult<&[u8], ZExpr, ZExprDeserialError<&[u8]>> {
    //println!("de inp {:?}", i);
    let (i, size) = take(1 as usize)(i)?;
    let (is_atom, len_typ, typ_len, dat_len_len) = (
      ((size[0] & 0b1000_0000) >> 7) == 0,
      ((size[0] & 0b0100_0000) >> 6) == 1,
      ((size[0] & 0b0011_1000) >> 3) + 1,
      (size[0] & 0b111) + 1,
    );
    //println!("de is_atom {}", is_atom);
    //println!("de len_typ {}", len_typ);
    //println!("de typ_len {}", typ_len);
    //println!("de dat_len_len {}", dat_len_len);
    if is_atom {
      let (i_type, typ_code) = take(typ_len)(i)?;
      let (i, dat_len) = take(dat_len_len)(i_type)?;
      let dat_len = dat_len.iter().fold(0, |acc, &x| (acc * 256) + x as u64);
      let len = if len_typ { Some(dat_len) } else { None };
      let (_, typ) = match ZType::deserialize(typ_code, len) {
        Some(x) => Ok((i, x)),
        None => Err(Err::Error(ZExprDeserialError::InvalidZTypeCode(
          i_type,
          typ_code.to_owned(),
        ))),
      }?;
      let (i, dat) = take(dat_len)(i)?;
      Ok((i, ZExpr::Atom(typ, dat.to_owned())))
    } else {
      println!("de is_atom {}", is_atom);
      let (i, xs_len) = take(dat_len_len)(i)?;
      let xs_len = xs_len.iter().fold(0, |acc, &x| (acc * 256) + x as u64);
      let (i, xs) = count(ZExpr::deserialize, xs_len as usize)(i)?;
      Ok((i, ZExpr::Cons(xs)))
    }
  }
}

pub fn number_of_bytes(x: u64) -> u8 {
  let mut n: u32 = 1;
  let base: u64 = 256;
  while base.pow(n) <= x {
    n += 1;
  }
  return n as u8;
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum ZExprDeserialError<I> {
  InvalidZTypeCode(I, Vec<u8>),
  NomErr(I, ErrorKind),
}

impl<I> ZExprDeserialError<I> {
  pub fn rest(self) -> I {
    match self {
      Self::InvalidZTypeCode(i, _) => i,
      Self::NomErr(i, _) => i,
    }
  }
}

impl<I> ParseError<I> for ZExprDeserialError<I>
where
  I: InputLength,
  I: Clone,
{
  fn from_error_kind(input: I, kind: ErrorKind) -> Self {
    ZExprDeserialError::NomErr(input, kind)
  }

  fn append(i: I, k: ErrorKind, other: Self) -> Self {
    if i.clone().input_len() < other.clone().rest().input_len() {
      ZExprDeserialError::NomErr(i, k)
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
  use crate::ZExpr;
  use quickcheck::{Arbitrary, Gen, StdThreadGen};
  use rand::Rng;

  impl Arbitrary for ZExpr {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
      let gen_atom = g.gen_ratio(2, 3);
      if gen_atom {
        // TODO: rework generator to test type-length-indexing
        ZExpr::Atom(ZType::Bytes(None), Arbitrary::arbitrary(g))
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
    assert_eq!(a.serialize(), vec![0, 0, 1, 0]);
    assert_eq!(
      ZExpr::deserialize(&vec![0, 0, 1, 0]).unwrap(),
      (b"".as_ref(), a.clone())
    );
    let b = ZExpr::Atom(Bytes(Some(1)), vec![0]);
    assert_eq!(format!("{}", b), "vy:bytes8");
    assert_eq!(b.serialize(), vec![64, 0, 1, 0]);
    assert_eq!(
      ZExpr::deserialize(&vec![64, 0, 1, 0]).unwrap(),
      (b"".as_ref(), b.clone())
    );

    let c = ZExpr::Cons(vec![a.clone(), a.clone(), a.clone()]);
    assert_eq!(format!("{}", c), "(vy:bytes vy:bytes vy:bytes)");
    assert_eq!(parse(&format!("{}", c)), Ok(("", c.clone())));
    assert_eq!(c.serialize(), vec![128, 3, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0]);
    assert_eq!(
      ZExpr::deserialize(&vec![128, 3, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0])
        .unwrap(),
      (b"".as_ref(), c.clone())
    );
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
  #[quickcheck]
  fn zexpr_serial_deserial(x: ZExpr) -> bool {
    match ZExpr::deserialize(&ZExpr::serialize(&x)) {
      Ok((_, y)) => x == y,
      _ => false,
    }
  }
}
