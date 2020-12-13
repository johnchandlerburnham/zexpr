use nom::error::ErrorKind;
use nom::error::ParseError;
use zexpr::{zbase, zbase::ZBase::*, ztype};

fn main() {
  let x = zbase::encode(Z2, "f");
  println!("{:?}", x);
  let x = zbase::decode("b1");
  println!("{:?}", x);
  println!("{:?}", ztype::parse("bytes32"));
  println!("{:?}", ztype::parse("bytes1"));
  println!(
    "{:?}",
    ztype::parse_index("29323421293847123984723198379874982174")
  );

  let t = ztype::ZTypeError::UnknownType("foo", String::from("bar"));
  println!("{:?}", ParseError::append("fo", ErrorKind::Tag, t));
  //("foo", nom::);
  //println!("{:?}", ztype::parse("bytes3122"));
  //println!("{}", ZType::Bytes(Some(4)));
  //println!("{:?}", ZType::Bytes(Some(4)).code());
  //println!("{:?}", ztype::parse("bytes8"));
}
