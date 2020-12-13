//use core::fmt;
//
//#[derive(PartialEq, Eq, Clone, Copy, Debug)]
//pub enum ZBase {
//  Binary,
//  Octal,
//  Decimal,
//  Hexadecimal,
//  Base32Z,
//  Base58Btc,
//  Base64Url,
//}
//
//impl Default for ZBase {
//  fn default() -> Self {
//    Self::Base32Z
//  }
//}
//
///// Error types
//#[derive(PartialEq, Eq, Clone, Debug)]
//pub enum ZBaseError {
//  /// Unknown print code.
//  UnknownCode(char),
//  /// Invalid string.
//  InvalidEncoding,
//}
//
//impl fmt::Display for ZBaseError {
//  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//    match self {
//      ZBaseError::UnknownCode(code) => {
//        write!(f, "Unknown ZBase code: {}", code)
//      }
//      ZBaseError::InvalidEncoding => write!(f, "Invalid ZBase encoding"),
//    }
//  }
//}
//
//impl std::error::Error for ZBaseError {}
//
//impl From<base_x::DecodeError> for ZBaseError {
//  fn from(_: base_x::DecodeError) -> Self {
//    Self::InvalidEncoding
//  }
//}
//
//pub const BASE2: &str = "01";
//pub const BASE8: &str = "01234567";
//pub const BASE10: &str = "0123456789";
//pub const BASE16: &str = "0123456789abcdef";
//pub const BASE32Z: &str = "ybndrfg8ejkmcpqxot1uwisza345h769";
//pub const BASE58BTC: &str =
//  "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
//pub const BASE64URL: &str =
//  "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
//
//impl ZBase {
//  pub fn from_code(code: char) -> Result<Self, ZBaseError> {
//    match code {
//      'b' => Ok(Self::Binary),
//      'o' => Ok(Self::Octal),
//      'd' => Ok(Self::Decimal),
//      'x' => Ok(Self::Hexadecimal),
//      'v' => Ok(Self::Base32Z),
//      '_' => Ok(Self::Base58Btc),
//      '~' => Ok(Self::Base64Url),
//      _ => Err(ZBaseError::UnknownCode(code)),
//    }
//  }
//
//  /// Get the code corresponding to the base algorithm.
//  pub fn code(&self) -> char {
//    match self {
//      Self::Binary => 'b',
//      Self::Octal => 'o',
//      Self::Decimal => 'd',
//      Self::Hexadecimal => 'x',
//      Self::Base32Z => 'v',
//      Self::Base58Btc => '_',
//      Self::Base64Url => '~',
//    }
//  }
//
//  /// Encode the given byte slice to base string.
//  pub fn encode<I: AsRef<[u8]>>(&self, input: I) -> String {
//    match self {
//      Self::Binary => base_x::encode(BASE2, input.as_ref()),
//      Self::Octal => base_x::encode(BASE8, input.as_ref()),
//      Self::Decimal => base_x::encode(BASE10, input.as_ref()),
//      Self::Hexadecimal => base_x::encode(BASE16, input.as_ref()),
//      Self::Base32Z => base_x::encode(BASE32Z, input.as_ref()),
//      Self::Base58Btc => base_x::encode(BASE58BTC, input.as_ref()),
//      Self::Base64Url => base_x::encode(BASE64URL, input.as_ref()),
//    }
//  }
//
//  /// Decode the base string.
//  pub fn decode<I: AsRef<str>>(&self, input: I) -> Result<Vec<u8>, ZBaseError> {
//    match self {
//      Self::Binary => Ok(base_x::decode(BASE2, input.as_ref())?),
//      Self::Octal => Ok(base_x::decode(BASE8, input.as_ref())?),
//      Self::Decimal => Ok(base_x::decode(BASE10, input.as_ref())?),
//      Self::Hexadecimal => Ok(base_x::decode(BASE16, input.as_ref())?),
//      Self::Base32Z => Ok(base_x::decode(BASE32Z, input.as_ref())?),
//      Self::Base58Btc => Ok(base_x::decode(BASE58BTC, input.as_ref())?),
//      Self::Base64Url => Ok(base_x::decode(BASE64URL, input.as_ref())?),
//    }
//  }
//}
//
//pub fn decode<T: AsRef<str>>(input: T) -> Result<(ZBase, Vec<u8>), ZBaseError> {
//  let input = input.as_ref();
//  let code = input.chars().next().ok_or(ZBaseError::InvalidEncoding)?;
//  println!("{:?}", code);
//  println!("{:?}", code.len_utf8());
//  //println!("{:?}", BASE2.decode("b01".as_ref().as_bytes()?);
//  let base = ZBase::from_code(code)?;
//  println!("{:?}", base);
//  let decoded = base.decode(&input[code.len_utf8()..])?;
//  Ok((base, decoded))
//}
//
//pub fn encode<T: AsRef<[u8]>>(base: ZBase, input: T) -> String {
//  let input = input.as_ref();
//  let mut encoded = base.encode(input.as_ref());
//  encoded.insert(0, base.code());
//  encoded
//}
//
//#[cfg(test)]
//mod tests {
//  use super::*;
//  use quickcheck::{Arbitrary, Gen};
//  use rand::Rng;
//
//  impl Arbitrary for ZBase {
//    fn arbitrary<G: Gen>(g: &mut G) -> Self {
//      let x: u32 = g.gen();
//      match x % 7 {
//        0 => ZBase::Binary,
//        1 => ZBase::Octal,
//        2 => ZBase::Decimal,
//        3 => ZBase::Hexadecimal,
//        4 => ZBase::Base32Z,
//        5 => ZBase::Base58Btc,
//        6 => ZBase::Base64Url,
//        _ => panic!("impossible"),
//      }
//    }
//  }
//  #[test]
//  fn it_works() {
//    assert_eq!(2 + 2, 4);
//  }
//
//  #[quickcheck]
//  fn zprint_code(x: ZBase) -> bool {
//    match ZBase::from_code(x.code()) {
//      Ok(y) => x == y && y.code() == x.code(),
//      _ => false,
//    }
//  }
//  #[quickcheck]
//  fn zprint_string(x: ZBase, s: String) -> bool {
//    match decode(encode(x, s.clone())) {
//      Ok((y, s2)) => x == y && s.into_bytes() == s2,
//      _ => false,
//    }
//  }
//}
