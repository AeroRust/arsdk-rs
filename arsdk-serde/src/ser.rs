use serde::{ser, Serialize};

use crate::error::{Error, Result};
use scroll::{ctx::TryIntoCtx, Pwrite};
use std::ffi::CString;

pub struct Serializer {
    // This string starts empty and JSON is appended as values are serialized.
    output: Vec<u8>,
}

impl Serializer {
    pub fn write<T: scroll::ctx::TryIntoCtx<scroll::Endian, Error = scroll::Error>>(
        &mut self,
        value: T,
    ) -> Result<()> {
        // TODO: Check if this buffer is enough
        let mut buf = [0_u8; 256];
        let actual_written = buf.gwrite_with(value, &mut 0, scroll::LE)?;

        self.output.extend_from_slice(&buf[..actual_written]);

        Ok(())
    }
}

pub fn to_bytes<T>(value: &T) -> Result<Vec<u8>>
where
    T: Serialize,
{
    let mut serializer = Serializer { output: Vec::new() };
    value.serialize(&mut serializer)?;
    Ok(serializer.output)
}

// TODO: Impl pub fn to_writer?

impl<'a> ser::Serializer for &'a mut Serializer {
    // The output type produced by this `Serializer` during successful
    // serialization. Most serializers that produce text or binary output should
    // set `Ok = ()` and serialize into an `io::Write` or buffer contained
    // within the `Serializer` instance, as happens here. Serializers that build
    // in-memory data structures may be simplified by using `Ok` to propagate
    // the data structure around.
    type Ok = ();

    // The error type when some error occurs during serialization.
    type Error = Error;

    // Associated types for keeping track of additional state while serializing
    // compound data structures like sequences and maps. In this case no
    // additional state is required beyond what is already stored in the
    // Serializer struct.
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<()> {
        let byte: u8 = v.into();

        self.write(byte)
    }

    fn serialize_i8(self, v: i8) -> Result<()> {
        self.write(v)
    }

    fn serialize_i16(self, v: i16) -> Result<()> {
        self.write(v)
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        self.write(v)
    }

    fn serialize_i64(self, v: i64) -> Result<()> {
        self.write(v)
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        self.write(v)
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        self.write(v)
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        self.write(v)
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        self.write(v)
    }

    fn serialize_f32(self, _v: f32) -> Result<()> {
        todo!()
    }

    fn serialize_f64(self, _v: f64) -> Result<()> {
        todo!()
    }

    // Serialize a char as a single-character string. Other formats may
    // represent this differently.
    fn serialize_char(self, v: char) -> Result<()> {
        self.serialize_str(&v.to_string())
    }

    // This only works for strings that don't require escape sequences but you
    // get the idea. For example it would emit invalid JSON if the input string
    // contains a '"' character.
    fn serialize_str(self, v: &str) -> Result<()> {
        // TODO: Handle error
        let cstring = CString::new(v).expect("Should create");
        self.output.extend_from_slice(cstring.to_bytes());

        Ok(())
    }

    // Serialize a byte array as an array of bytes. Could also use a base64
    // string here. Binary formats will typically represent byte arrays more
    // compactly.
    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        self.output.extend_from_slice(v);

        Ok(())
    }

    // An absent optional is represented as the JSON `null`.
    fn serialize_none(self) -> Result<()> {
        self.serialize_unit()
    }

    fn serialize_some<T>(self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    /// Do nothing!
    fn serialize_unit(self) -> Result<()> {
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        self.serialize_unit()
    }

    // When serializing a unit variant (or any other kind of variant), formats
    // can choose whether to keep track of it by index or by name. Binary
    // formats typically use the index of the variant and human-readable formats
    // typically use the name.
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
    ) -> Result<()> {
        self.write(variant_index)
    }

    // As is done here, serializers are encouraged to treat newtype structs as
    // insignificant wrappers around the data they contain.
    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    // Note that newtype variant (and all of the other variant serialization
    // methods) refer exclusively to the "externally tagged" enum
    // representation.
    //
    // Serialize this to JSON in externally tagged form as `{ NAME: VALUE }`.
    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    // Now we get to the serialization of compound types.
    //
    // The start of the sequence, each value, and the end are three separate
    // method calls. This one is responsible only for serializing the start,
    // which in JSON is `[`.
    //
    // The length of the sequence may or may not be known ahead of time. This
    // doesn't make a difference in JSON because the length is not represented
    // explicitly in the serialized form. Some serializers may only be able to
    // support sequences for which the length is known up front.
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(self)
    }

    // Tuples look just like sequences in JSON. Some formats may be able to
    // represent tuples more efficiently by omitting the length, since tuple
    // means that the corresponding `Deserialize implementation will know the
    // length without needing to look at the serialized data.
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        self.serialize_seq(Some(len))
    }

    // Tuple structs look just like sequences in JSON.
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        self.serialize_seq(Some(len))
    }

    // Tuple variants are represented in JSON as `{ NAME: [DATA...] }`. Again
    // this method is only responsible for the externally tagged representation.
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        // TODO: SOMEHOW get how we should serialize this variant
        // We need to handle a prefixed (repr-like) attribute
        variant.serialize(&mut *self)?;

        Ok(self)
    }

    // Maps are represented in JSON as `{ K: V, K: V, ... }`.
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Ok(self)
    }

    // Structs look just like maps in JSON. In particular, JSON requires that we
    // serialize the field names of the struct. Other formats may be able to
    // omit the field names when serializing structs because the corresponding
    // Deserialize implementation is required to know what the keys are without
    // looking at the serialized data.
    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        self.serialize_map(Some(len))
    }

    // Struct variants are represented in JSON as `{ NAME: { K: V, ... } }`.
    // This is the externally tagged representation.
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        variant.serialize(&mut *self)?;

        Ok(self)
    }
}

// The following 7 impls deal with the serialization of compound types like
// sequences and maps. Serialization of such types is begun by a Serializer
// method and followed by zero or more calls to serialize individual elements of
// the compound type and one call to end the compound type.
//
// This impl is SerializeSeq so these methods are called after `serialize_seq`
// is called on the Serializer.
impl<'a> ser::SerializeSeq for &'a mut Serializer {
    // Must match the `Ok` type of the serializer.
    type Ok = ();
    // Must match the `Error` type of the serializer.
    type Error = Error;

    // Serialize a single element of the sequence.
    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    // Close the sequence.
    fn end(self) -> Result<()> {
        Ok(())
    }
}

// Same thing but for tuples.
impl<'a> ser::SerializeTuple for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

// Same thing but for tuple structs.
impl<'a> ser::SerializeTupleStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

// Tuple variants are a little different. Refer back to the
// `serialize_tuple_variant` method above:
//
//    self.output += "{";
//    variant.serialize(&mut *self)?;
//    self.output += ":[";
//
// So the `end` method in this impl is responsible for closing both the `]` and
// the `}`.
impl<'a> ser::SerializeTupleVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

// Some `Serialize` types are not able to hold a key and value in memory at the
// same time so `SerializeMap` implementations are required to support
// `serialize_key` and `serialize_value` individually.
//
// There is a third optional method on the `SerializeMap` trait. The
// `serialize_entry` method allows serializers to optimize for the case where
// key and value are both available simultaneously. In JSON it doesn't make a
// difference so the default behavior for `serialize_entry` is fine.
impl<'a> ser::SerializeMap for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, _key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        // Keys are not serialized in Arsdk-rs
        Ok(())
    }

    // It doesn't make a difference whether the colon is printed at the end of
    // `serialize_key` or at the beginning of `serialize_value`. In this case
    // the code is a bit simpler having it here.
    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

// Structs are like maps in which the keys are constrained to be compile-time
// constant strings.
impl<'a> ser::SerializeStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        // TODO: Check if the struct fields will be serialized in order in which they have been defined on the struct or not.
        // We need to them in the same order to keep the ordering of the serialization
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

// Similar to `SerializeTupleVariant`, here the `end` method is responsible for
// closing both of the curly braces opened by `serialize_struct_variant`.
impl<'a> ser::SerializeStructVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        // TODO: Again skip the key and check later if the ordering will be the same
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_values() {
        assert_eq!(to_bytes(&5_u8).unwrap(), vec![5_u8]);
    }

    mod result_tuple {
        use scroll::Pread;
        use serde::{Deserialize, Serializer, Serialize};
        use std::convert::TryFrom;
        use crate::ser::Error;

        pub type Common = u8;
        pub type ArDrone3 = u16;

        pub type Tuple<V> = (V, Vec<u8>);
        pub type TupleResult<V> = ResultDef<Tuple<V>, Error>;

        #[derive(Debug, Serialize)]
        #[serde(remote = "std::result::Result")]
        struct ResultDef<V, E>(std::result::Result<V, E>);

        impl<V, E> Serialize for ResultDef<V, E>
        where
            V: Serialize,
        {
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                self.0?
            }
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        #[serde(try_from = "Tuple<u8>", into = "TupleResult<u8>")]
        pub enum Feature {
            Common(Common),
            ArDrone3(ArDrone3),
        }

        // TODO: Serde doesn't allow for `TryInto`, can we use a Result instead?
        impl Into<TupleResult<u8>> for Feature {
            fn into(self) -> TupleResult<u8> {
                let mut buf = [0_u8; 2048];
                let mut offset = 0;

                match self {
                    Feature::Common(common) => {
                        buf.gwrite_with(common, &mut offset, scroll::LE)?;

                        ResultDef(Ok((0, buf[..offset - 1].to_vec())))
                    }
                    Feature::ArDrone3(ardrone3) => {
                        buf.gwrite_with(ardrone3, &mut offset, scroll::LE)?;

                        ResultDef(Ok((1, buf[..offset - 1].to_vec())))
                    }
                }
            }
        }

        impl TryFrom<Tuple<u8>> for Feature {
            type Error = Error;

            fn try_from(tuple: Tuple<u8>) -> Result<Self, Error> {
                let mut offset = 0;

                match tuple.0 {
                    0 => Ok(Self::Common(
                        tuple.1.gread_with::<Common>(&mut offset, scroll::LE)?,
                    )),
                    1 => Ok(Self::ArDrone3(
                        tuple.1.gread_with::<ArDrone3>(&mut offset, scroll::LE)?,
                    )),
                    _ => Err(Error::Message("Out of bound for Feature".to_string())),
                }
            }
        }

        #[test]
        /// Feature - ArDrone3 = 1_u8
        /// ArDrone3 - 4_u16
        fn test_enum_try_from_into() {
            let feature = Feature::ArDrone3(4);

            let actual = to_bytes(&feature).expect("Should serialize");

            let serialized = [1, 4];

            assert_eq!(actual, serialized);
        }
    }
    #[test]
    #[ignore = "We don't use Serde_repr for now"]
    fn test_serde_repr_enum() {
        // use serde_repr::Serialize_repr;

        // #[derive(Debug, Serialize_repr)]
        // #[repr(u8)]
        #[derive(Debug)]
        pub enum Anim {
            JumpStop,        // ARCOMMANDS_ID_JUMPINGSUMO_ANIMATIONS_CMD_JUMPSTOP = 0,
            JumpCancel,      // ARCOMMANDS_ID_JUMPINGSUMO_ANIMATIONS_CMD_JUMPCANCEL = 1,
            JumpLoad,        // ARCOMMANDS_ID_JUMPINGSUMO_ANIMATIONS_CMD_JUMPLOAD = 2,
            Jump,            // ARCOMMANDS_ID_JUMPINGSUMO_ANIMATIONS_CMD_JUMP = 3,
            SimpleAnimation, // ARCOMMANDS_ID_JUMPINGSUMO_ANIMATIONS_CMD_SIMPLEANIMATION = 4,
        }

        let anim = Anim::Jump;
        let expected: Vec<u8> = vec![3];
        // assert_eq!(to_bytes(&anim).unwrap(), expected);

        // let n = E::Newtype(1);
        // let expected = r#"{"Newtype":1}"#;
        // assert_eq!(to_string(&n).unwrap(), expected);

        // let t = E::Tuple(1, 2);
        // let expected = r#"{"Tuple":[1,2]}"#;
        // assert_eq!(to_string(&t).unwrap(), expected);

        // let s = E::Struct { a: 1 };
        // let expected = r#"{"Struct":{"a":1}}"#;
        // assert_eq!(to_string(&s).unwrap(), expected);
    }
}
