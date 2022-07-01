//! `CompoundTag` type to ease transition from `named-binary-tag` to `fastnbt` crate.
use flate2::read::GzDecoder;
use std::io::Read;
use std::collections::HashMap;
use fastnbt::Value;

pub struct CompoundTag {
    m: HashMap<String, Value>,
}

impl CompoundTag {
    pub fn from_bytes(data: &[u8]) -> Result<Self, fastnbt::error::Error> {
        let m = fastnbt::from_bytes(data)?;

        Ok(Self { m })
    }

    pub fn get_compound_tag(&self, name: &str) -> Result<Self, CompoundTagError> {
        match self.m.get(name) {
            Some(Value::Compound(m)) => Ok(Self { m: m.clone() }),
            Some(_) => Err(CompoundTagError::TagWrongType),
            None => Err(CompoundTagError::TagNotFound),
        }
    }

    pub fn get_compound_tag_vec(&self, name: &str) -> Result<Vec<Self>, CompoundTagError> {
        match self.m.get(name) {
            Some(Value::List(l)) => {
                let mut v = vec![];

                for x in l {
                    match x {
                        Value::Compound(m) => {
                            v.push(Self { m: m.clone() });
                        }
                        _ => {
                            return Err(CompoundTagError::TagWrongType);
                        }
                    }
                }

                Ok(v)
            }
            Some(_) => Err(CompoundTagError::TagWrongType),
            None => Err(CompoundTagError::TagNotFound),
        }
    }

    pub fn get_i32(&self, name: &str) -> Result<i32, CompoundTagError> {
        match self.m.get(name) {
            Some(Value::Int(x)) => Ok(*x),
            Some(_) => Err(CompoundTagError::TagWrongType),
            None => Err(CompoundTagError::TagNotFound),
        }
    }

    pub fn get_i64(&self, name: &str) -> Result<i64, CompoundTagError> {
        match self.m.get(name) {
            Some(Value::Long(x)) => Ok(*x),
            Some(_) => Err(CompoundTagError::TagWrongType),
            None => Err(CompoundTagError::TagNotFound),
        }
    }

    pub fn get_i8_vec(&self, name: &str) -> Result<&[i8], CompoundTagError> {
        match self.m.get(name) {
            Some(Value::ByteArray(x)) => Ok(x.as_ref()),
            Some(_) => Err(CompoundTagError::TagWrongType),
            None => Err(CompoundTagError::TagNotFound),
        }
    }

    pub fn get_i32_vec(&self, name: &str) -> Result<&[i32], CompoundTagError> {
        match self.m.get(name) {
            Some(Value::IntArray(x)) => Ok(x.as_ref()),
            Some(_) => Err(CompoundTagError::TagWrongType),
            None => Err(CompoundTagError::TagNotFound),
        }
    }

    pub fn get_str(&self, name: &str) -> Result<&str, CompoundTagError> {
        match self.m.get(name) {
            Some(Value::String(x)) => Ok(x),
            Some(_) => Err(CompoundTagError::TagWrongType),
            None => Err(CompoundTagError::TagNotFound),
        }
    }
}

#[derive(Debug)]
pub enum CompoundTagError {
    TagNotFound,
    TagWrongType,
}

#[derive(Debug)]
pub enum TagDecodeError {
    IoError(std::io::Error),
    FastNbtError(fastnbt::error::Error),
}

impl From<std::io::Error> for TagDecodeError {
    fn from(e: std::io::Error) -> Self {
        Self::IoError(e)
    }
}

impl From<fastnbt::error::Error> for TagDecodeError {
    fn from(e: fastnbt::error::Error) -> Self {
        Self::FastNbtError(e)
    }
}

pub fn read_gzip_compound_tag<R: Read>(reader: &mut R) -> Result<CompoundTag, TagDecodeError> {
    let mut gz = GzDecoder::new(reader);
    let mut v = vec![];
    gz.read_to_end(&mut v)?;
    let tag = CompoundTag::from_bytes(&v)?;

    Ok(tag)
}

