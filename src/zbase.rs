use core::fmt;

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

/// Error types
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum ZBaseError {
  /// Unknown print code.
  UnknownCode(char),
  /// Invalid string.
  InvalidEncoding,
}

impl fmt::Display for ZBaseError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      ZBaseError::UnknownCode(code) => {
        write!(f, "Unknown ZBase code: {}", code)
      }
      ZBaseError::InvalidEncoding => write!(f, "Invalid ZBase encoding"),
    }
  }
}

impl std::error::Error for ZBaseError {}

impl From<base_x::DecodeError> for ZBaseError {
  fn from(_: base_x::DecodeError) -> Self {
    Self::InvalidEncoding
  }
}

pub const BASE2: &str = "01";
pub const BASE8: &str = "01234567";
pub const BASE10: &str = "0123456789";
pub const BASE16: &str = "0123456789abcdef";
pub const BASE32Z: &str = "ybndrfg8ejkmcpqxot1uwisza345h769";
pub const BASE58BTC: &str =
  "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
pub const BASE64URL: &str =
  "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

impl ZBase {
  pub fn from_code(code: char) -> Result<Self, ZBaseError> {
    match code {
      'b' => Ok(Self::Z2),
      'o' => Ok(Self::Z8),
      'd' => Ok(Self::Z10),
      'x' => Ok(Self::Z16),
      'v' => Ok(Self::Z32),
      'I' => Ok(Self::Z58),
      '~' => Ok(Self::Z64),
      _ => Err(ZBaseError::UnknownCode(code)),
    }
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

  /// Encode the given byte slice to base string.
  pub fn encode<I: AsRef<[u8]>>(&self, input: I) -> String {
    match self {
      Self::Z2 => base_x::encode(BASE2, input.as_ref()),
      Self::Z8 => base_x::encode(BASE8, input.as_ref()),
      Self::Z10 => base_x::encode(BASE10, input.as_ref()),
      Self::Z16 => base_x::encode(BASE16, input.as_ref()),
      Self::Z32 => base_x::encode(BASE32Z, input.as_ref()),
      Self::Z58 => base_x::encode(BASE58BTC, input.as_ref()),
      Self::Z64 => base_x::encode(BASE64URL, input.as_ref()),
    }
  }

  /// Decode the base string.
  pub fn decode<I: AsRef<str>>(&self, input: I) -> Result<Vec<u8>, ZBaseError> {
    match self {
      Self::Z2 => Ok(base_x::decode(BASE2, input.as_ref())?),
      Self::Z8 => Ok(base_x::decode(BASE8, input.as_ref())?),
      Self::Z10 => Ok(base_x::decode(BASE10, input.as_ref())?),
      Self::Z16 => Ok(base_x::decode(BASE16, input.as_ref())?),
      Self::Z32 => Ok(base_x::decode(BASE32Z, input.as_ref())?),
      Self::Z58 => Ok(base_x::decode(BASE58BTC, input.as_ref())?),
      Self::Z64 => Ok(base_x::decode(BASE64URL, input.as_ref())?),
    }
  }
}

pub fn decode<T: AsRef<str>>(input: T) -> Result<(ZBase, Vec<u8>), ZBaseError> {
  let input = input.as_ref();
  let code = input.chars().next().ok_or(ZBaseError::InvalidEncoding)?;
  let base = ZBase::from_code(code)?;
  let decoded = base.decode(&input[code.len_utf8()..])?;
  Ok((base, decoded))
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
    match ZBase::from_code(x.code()) {
      Ok(y) => x == y && y.code() == x.code(),
      _ => false,
    }
  }
  #[quickcheck]
  fn zprint_string(x: ZBase, s: String) -> bool {
    match decode(encode(x, s.clone())) {
      Ok((y, s2)) => x == y && s.into_bytes() == s2,
      _ => false,
    }
  }
}
