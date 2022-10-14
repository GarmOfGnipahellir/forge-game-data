use crate::error::{Error, Result};
use serde::{de, forward_to_deserialize_any, Deserialize};

// pub struct Deserializer<'de> {
//     input: &'de str,
// }

// impl<'de> Deserializer<'de> {
//     pub fn from_str(input: &'de str) -> Self {
//         Self { input }
//     }
// }

// pub fn from_str<'a, T>(s: &'a str) -> Result<T>
// where
//     T: Deserialize<'a>,
// {
//     let mut deserializer = Deserializer::from_str(s);
//     let t = T::deserialize(&mut deserializer)?;
//     if deserializer.input.is_empty() {
//         Ok(t)
//     } else {
//         Err(Error::TrailingCharacters)
//     }
// }

// impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
//     type Error = Error;

//     fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
//     where
//         V: de::Visitor<'de>,
//     {
//         todo!()
//         visitor.visi
//     }

//     forward_to_deserialize_any! {
//         bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
//         bytes byte_buf option unit unit_struct newtype_struct seq tuple
//         tuple_struct map struct enum identifier ignored_any
//     }
// }
