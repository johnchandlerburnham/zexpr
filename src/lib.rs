#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;
#[cfg(test)]
extern crate rand;

extern crate nom;

//mod de;
//mod error;
//mod ser;
pub mod zbase;
pub mod zexpr;
pub mod ztype;

//pub use crate::{
//  de::{from_str, Deserializer},
//  error::{Error, Result},
//  ser::{to_string, Serializer},
//  zbase::{decode, encode, *},
//};

// pub enum ZExpr {
//  Atom(ZAtom),
//  Cons(Vec<ZExpr>),
//}
//
//#[derive(Debug)]
// pub struct ZAtom {
//  typ: Vec<u8>,
//  dat: Vec<u8>,
//}
//
//#[derive(Debug)]
// pub enum DeserialErr {
//  UnexpectedEndOfInput,
//  ExpectedAtomCode(u8),
//}
// impl ZAtom {
//  pub fn new(typ: Vec<u8>, dat: Vec<u8>) -> ZAtom {
//    if typ.len() > 8 || typ.len() < 1 {
//      panic!(
//        "Cannot make ZAtom with type smaller than 8 bits or larger than 64 \
//         bits"
//      )
//    }
//    else {
//      return ZAtom { typ, dat };
//    }
//  }
//
//  pub fn serialize(&self) -> Vec<u8> {
//    let typ_len: u8 = self.typ.len() as u8;
//    let dat_len: u64 = self.dat.len() as u64;
//    let dat_len_len: u8 = number_of_bytes(dat_len);
//
//    let dat_len: Vec<u8> = dat_len.to_le_bytes()[0..(dat_len_len as usize)]
//      .to_vec()
//      .into_iter()
//      .rev()
//      .collect();
//
//    let size_byte: u8 =
//      (0b0011_1111) & ((typ_len - 1) << 3) | (dat_len_len - 1);
//
//    let mut ret = vec![];
//
//    ret.extend(self.dat.iter().cloned().collect::<Vec<u8>>());
//    ret.extend(dat_len);
//    ret.extend(self.typ.iter().cloned().collect::<Vec<u8>>());
//    ret.extend(vec![size_byte]);
//
//    return ret;
//  }
//
//  pub fn deserialize(mut bs: Vec<u8>) -> Result<ZAtom, DeserialErr> {
//    let size_byte = bs.pop().ok_or(DeserialErr::UnexpectedEndOfInput)?;
//    ((size_byte & 0b1100000) == 0b00)
//      .then_some(())
//      .ok_or(DeserialErr::ExpectedAtomCode(size_byte))?;
//
//    let (typ_len, dat_len_len) =
//      (((size_byte & 0b0011_1000) >> 3) + 1, (size_byte & 0b111) + 1);
//
//    println!("typ_len: {}", typ_len);
//    println!("dat_len: {}", dat_len_len);
//
//    (bs.len() > typ_len as usize)
//      .then_some(())
//      .ok_or(DeserialErr::UnexpectedEndOfInput)?;
//
//    let typ = bs.split_off(bs.len() - typ_len as usize);
//
//    println!("typ: {:?}", typ);
//
//    (bs.len() > dat_len_len as usize)
//      .then_some(())
//      .ok_or(DeserialErr::UnexpectedEndOfInput)?;
//
//    let dat_len = bs.split_off(bs.len() - dat_len_len as usize);
//    println!("dat_len: {:?}", dat_len);
//    let dat_len = dat_len.iter().fold(0, |acc, &x| (acc * 256) + x as usize);
//    println!("dat_len: {:?}", dat_len);
//
//    (bs.len() >= dat_len as usize)
//      .then_some(())
//      .ok_or(DeserialErr::UnexpectedEndOfInput)?;
//
//    let dat = bs.split_off(bs.len() - dat_len as usize);
//    return Ok(ZAtom::new(typ, dat));
//  }
//}
// pub fn number_of_bytes(x: u64) -> u8 {
//  let mut n: u32 = 1;
//  let base: u64 = 256;
//  while base.pow(n) <= x {
//    n += 1;
//  }
//  return n as u8;
//}
//
////#[cfg(test)]
//// mod tests {
////    quickcheck !
////
////
//// }
