use crate::{
    error::{Error, Result},
    options::NbtOptions,
    value::Value,
};
use byteorder::{BigEndian, WriteBytesExt};
use serde::ser::{self, Serialize};
use std::io::Write;

pub struct NbtWriter<W> {
    writer: W,
    options: NbtOptions,
}

impl<W: Write> NbtWriter<W> {
    pub const fn new(writer: W) -> Self {
        Self::new_with_options(writer, NbtOptions::new())
    }

    pub const fn new_with_options(writer: W, options: NbtOptions) -> Self {
        Self { writer, options }
    }

    fn write_string(&mut self, s: &str) -> Result<()> {
        let bytes = s.as_bytes();
        let len = u16::try_from(bytes.len())
            .map_err(|_| Error::Message("String too large (65535 bytes max)".into()))?;
        self.writer.write_u16::<BigEndian>(len)?;
        self.writer.write_all(bytes)?;
        Ok(())
    }

    fn write_value(&mut self, value: &Value) -> Result<()> {
        match value {
            Value::Byte(v) => self.writer.write_i8(*v)?,
            Value::Short(v) => self.writer.write_i16::<BigEndian>(*v)?,
            Value::Int(v) => self.writer.write_i32::<BigEndian>(*v)?,
            Value::Long(v) => self.writer.write_i64::<BigEndian>(*v)?,
            Value::Float(v) => self.writer.write_f32::<BigEndian>(*v)?,
            Value::Double(v) => self.writer.write_f64::<BigEndian>(*v)?,
            Value::ByteArray(v) => {
                let len = i32::try_from(v.len())
                    .map_err(|_| Error::Message("ByteArray too large".into()))?;
                self.writer.write_i32::<BigEndian>(len)?;
                self.writer.write_all(v)?;
            }
            Value::String(v) => self.write_string(v)?,
            Value::List(list) => {
                if list.is_empty() {
                    self.writer.write_u8(0)?; // TAG_End see french docs: https://fr.minecraft.wiki/w/Format_NBT
                    self.writer.write_i32::<BigEndian>(0)?;
                } else {
                    let first_id = list
                        .first()
                        .map(Value::id)
                        .ok_or_else(|| Error::Message("Empty list".into()))?;

                    let is_heterogenus = list.iter().any(|elem| elem.id() != first_id);

                    if is_heterogenus {
                        if self.options.is_dynamic_lists() {
                            // Write as List of Compounds
                            self.writer.write_u8(10)?;
                            let len = i32::try_from(list.len())
                                .map_err(|_| Error::Message("List too large".into()))?;
                            self.writer.write_i32::<BigEndian>(len)?;

                            for elem in list {
                                // 1. Write Tag ID of element
                                self.writer.write_u8(elem.id())?;
                                // 2. Write Name (empty string)
                                self.write_string("")?;
                                // 3. Write Value
                                self.write_value(elem)?;
                                // 4. Write TAG_End see french docs: https://fr.minecraft.wiki/w/Format_NBT
                                self.writer.write_u8(0)?;
                            }
                        } else {
                            return Err(Error::Message("Heterogenous lists are not supported in binary NBT unless dynamic_lists option is enabled".to_string()));
                        }
                    } else {
                        // Homogenous list
                        self.writer.write_u8(first_id)?;
                        let len = i32::try_from(list.len())
                            .map_err(|_| Error::Message("List too large".into()))?;
                        self.writer.write_i32::<BigEndian>(len)?;
                        for elem in list {
                            self.write_value(elem)?;
                        }
                    }
                }
            }
            Value::Compound(map) => {
                for (name, val) in map {
                    self.writer.write_u8(val.id())?;
                    self.write_string(name)?;
                    self.write_value(val)?;
                }
                self.writer.write_u8(0)?; // TAG_End see french docs: https://fr.minecraft.wiki/w/Format_NBT
            }
            Value::IntArray(v) => {
                let len = i32::try_from(v.len())
                    .map_err(|_| Error::Message("IntArray too large".into()))?;
                self.writer.write_i32::<BigEndian>(len)?;
                for &i in v {
                    self.writer.write_i32::<BigEndian>(i)?;
                }
            }
            Value::LongArray(v) => {
                let len = i32::try_from(v.len())
                    .map_err(|_| Error::Message("LongArray too large".into()))?;
                self.writer.write_i32::<BigEndian>(len)?;
                for &l in v {
                    self.writer.write_i64::<BigEndian>(l)?;
                }
            }
        }
        Ok(())
    }

    pub(crate) fn write_root(&mut self, name: &str, value: &Value) -> Result<()> {
        self.writer.write_u8(value.id())?;
        if !self.options.is_nameless_root() {
            self.write_string(name)?;
        }
        self.write_value(value)?;
        Ok(())
    }
}

/// Serializes a value to NBT `Value`.
pub fn to_value<T: Serialize>(value: T) -> Result<Value> {
    value.serialize(Serializer)
}

/// Serializes an NBT value to a writer.
pub fn to_writer<W: Write, T: Serialize>(
    writer: W,
    value: &T,
    root_name: Option<&str>,
) -> Result<()> {
    let nbt_value = to_value(value)?;
    let mut encoder = NbtWriter::new(writer);
    encoder.write_root(root_name.unwrap_or(""), &nbt_value)
}

/// Serializes an NBT `Value` directly to a writer.
pub fn to_writer_value<W: Write>(writer: W, value: &Value, root_name: Option<&str>) -> Result<()> {
    let mut encoder = NbtWriter::new(writer);
    encoder.write_root(root_name.unwrap_or(""), value)
}

/// Serializes an NBT value to a writer with options.
pub fn to_writer_with_options<W: Write, T: Serialize>(
    writer: W,
    value: &T,
    root_name: Option<&str>,
    options: NbtOptions,
) -> Result<()> {
    let nbt_value = to_value(value)?;
    let mut encoder = NbtWriter::new_with_options(writer, options);
    encoder.write_root(root_name.unwrap_or(""), &nbt_value)
}

/// Serializes an NBT `Value` to a writer with options.
pub fn to_writer_value_with_options<W: Write>(
    writer: W,
    value: &Value,
    root_name: Option<&str>,
    options: NbtOptions,
) -> Result<()> {
    let mut encoder = NbtWriter::new_with_options(writer, options);
    encoder.write_root(root_name.unwrap_or(""), value)
}

struct SerializeSeq {
    vec: Vec<Value>,
    variant_name: Option<String>,
}

impl ser::SerializeSeq for SerializeSeq {
    type Ok = Value;
    type Error = Error;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<()> {
        self.vec.push(value.serialize(Serializer)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        // If variant_name is set, wrap in compound
        let val = Value::List(self.vec);
        if let Some(variant) = self.variant_name {
            let mut map = indexmap::IndexMap::new();
            map.insert(variant, val);
            Ok(Value::Compound(map))
        } else {
            Ok(val)
        }
    }
}

impl ser::SerializeTuple for SerializeSeq {
    type Ok = Value;
    type Error = Error;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<()> {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok> {
        ser::SerializeSeq::end(self)
    }
}

impl ser::SerializeTupleStruct for SerializeSeq {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<()> {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok> {
        ser::SerializeSeq::end(self)
    }
}

impl ser::SerializeTupleVariant for SerializeSeq {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<()> {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok> {
        ser::SerializeSeq::end(self)
    }
}

struct SerializeMap {
    map: indexmap::IndexMap<String, Value>,
    next_key: Option<String>,
    variant_name: Option<String>,
}

impl ser::SerializeMap for SerializeMap {
    type Ok = Value;
    type Error = Error;

    fn serialize_key<T: ?Sized + Serialize>(&mut self, key: &T) -> Result<()> {
        // Key must be string
        // We serialize key to Value, expect String
        let key_val = key.serialize(Serializer)?;
        if let Value::String(s) = key_val {
            self.next_key = Some(s);
            Ok(())
        } else {
            Err(Error::Message("Map key must be a string".into()))
        }
    }

    fn serialize_value<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<()> {
        let key = self
            .next_key
            .take()
            .ok_or_else(|| Error::Message("Map value without key".into()))?;
        let val = value.serialize(Serializer)?;
        self.map.insert(key, val);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        let val = Value::Compound(self.map);
        if let Some(variant) = self.variant_name {
            let mut map = indexmap::IndexMap::new();
            map.insert(variant, val);
            Ok(Value::Compound(map))
        } else {
            Ok(val)
        }
    }
}

impl ser::SerializeStruct for SerializeMap {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<()> {
        let val = value.serialize(Serializer)?;
        self.map.insert(key.into(), val);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        ser::SerializeMap::end(self)
    }
}

impl ser::SerializeStructVariant for SerializeMap {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<()> {
        let val = value.serialize(Serializer)?;
        self.map.insert(key.into(), val);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        ser::SerializeMap::end(self)
    }
}

struct Serializer;

impl ser::Serializer for Serializer {
    type Ok = Value;
    type Error = Error;
    type SerializeSeq = SerializeSeq;
    type SerializeTuple = SerializeSeq;
    type SerializeTupleStruct = SerializeSeq;
    type SerializeTupleVariant = SerializeSeq;
    type SerializeMap = SerializeMap;
    type SerializeStruct = SerializeMap;
    type SerializeStructVariant = SerializeMap;

    fn serialize_bool(self, v: bool) -> std::result::Result<Self::Ok, Self::Error> {
        Ok(Value::Byte(i8::from(v)))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok> {
        Ok(Value::Byte(v))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok> {
        Ok(Value::Short(v))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok> {
        Ok(Value::Int(v))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok> {
        Ok(Value::Long(v))
    }

    fn serialize_u8(self, _: u8) -> Result<Self::Ok> {
        Err(Error::Message(
            "Cannot serialize unsigned number to NBT Value".into(),
        ))
    }

    fn serialize_u16(self, _: u16) -> Result<Self::Ok> {
        Err(Error::Message(
            "Cannot serialize unsigned number to NBT Value".into(),
        ))
    }

    fn serialize_u32(self, _: u32) -> Result<Self::Ok> {
        Err(Error::Message(
            "Cannot serialize unsigned number to NBT Value".into(),
        ))
    }

    fn serialize_u64(self, _: u64) -> Result<Self::Ok> {
        Err(Error::Message(
            "Cannot serialize unsigned number to NBT Value".into(),
        ))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok> {
        Ok(Value::Float(v))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok> {
        Ok(Value::Double(v))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok> {
        Ok(Value::String(v.to_string()))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok> {
        Ok(Value::String(v.to_string()))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok> {
        Ok(Value::ByteArray(v.to_vec()))
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        Err(Error::Message("Cannot serialize None to NBT Value".into()))
    }

    fn serialize_some<T: ?Sized + Serialize>(self, value: &T) -> Result<Self::Ok> {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        Ok(Value::Compound(indexmap::IndexMap::new()))
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok> {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok> {
        let mut map = indexmap::IndexMap::new();
        map.insert(variant.into(), value.serialize(self)?);
        Ok(Value::Compound(map))
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(SerializeSeq {
            vec: Vec::with_capacity(len.unwrap_or(0)),
            variant_name: None,
        })
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Ok(SerializeSeq {
            vec: Vec::with_capacity(len),
            variant_name: Some(variant.to_string()),
        })
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Ok(SerializeMap {
            map: indexmap::IndexMap::new(),
            next_key: None,
            variant_name: None,
        })
    }

    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Ok(SerializeMap {
            map: indexmap::IndexMap::new(),
            next_key: None,
            variant_name: Some(variant.to_string()),
        })
    }
}

/// Serializes an NBT value to a byte vector.
pub fn to_bytes<T: Serialize>(values: &T, root_name: Option<&str>) -> Result<Vec<u8>> {
    to_value(values)?.to_bytes(
        crate::io::CompressionType::None,
        NbtOptions::new(),
        root_name,
    )
}

pub fn to_bytes_with_options<T: Serialize>(
    values: &T,
    root_name: Option<&str>,
    options: NbtOptions,
) -> Result<Vec<u8>> {
    to_value(values)?.to_bytes(crate::io::CompressionType::None, options, root_name)
}
