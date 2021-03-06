//const ASN1_CONSTRUCTED_FLAG: u8 = 0x20;
//
//#[repr(u8)]
//pub enum Tag {
//    Bool = 0x1,
//    Integer = 0x2,
//    BitString = 0x3,
//    OctetString = 0x4,
//    ObjectIdentifier = 0x6,
//    PrintableString = 0x13,
//    UTCTime = 0x17,
//    Sequence = (0x10 | ASN1_CONSTRUCTED_FLAG),
//}

#[derive(Debug)]
pub enum IntToEnumError {
    InvalidU8(u8),
    InvalidU16(u16),
}

#[macro_export]
macro_rules! error_if (
  ($i:expr, $cond:expr, $err:expr) => (
    {
      if $cond {
        IResult::Error($err)
      } else {
        IResult::Done($i, ())
      }
    }
  );
  ($i:expr, $cond:expr, $err:expr) => (
    error!($i, $cond, $err);
  );
);



pub fn bytes_to_u64(s: &[u8]) -> Result<u64, &'static str> {
    let mut u = 0;

    for &c in s {
        u *= 256;
        u += c as u64;
    }

    Ok(u)
}

#[macro_use]
macro_rules! parse_hex_to_u64 (
    ( $i:expr, $size:expr ) => (
        map_res!($i, take!(($size as usize)), $crate::bytes_to_u64)
    );
);

