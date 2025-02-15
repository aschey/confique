//! Types for [`Config::META`][super::Config::META]. Represent information about
//! a configuration type.

use core::fmt;

// TODO: having all these fields public make me uncomfortable. For now it's
// fine, but before reaching 1.0 I need to figure out how to allow future
// additions without breaking stuff.

/// Root type.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Meta {
    /// The type (struct) name.
    pub name: &'static str,

    /// Doc comments.
    pub doc: &'static [&'static str],

    pub fields: &'static [Field],
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Field {
    pub name: &'static str,
    pub doc: &'static [&'static str],
    pub kind: FieldKind,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FieldKind {
    Leaf {
        env: Option<&'static str>,
        kind: LeafKind,
    },
    Nested {
        meta: &'static Meta,
    },
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LeafKind {
    /// A leaf field with a non `Option<_>` type.
    Required { default: Option<Expr> },
    /// A leaf field with an `Option<_>` type.
    Optional,
}

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum Expr {
    Str(&'static str),
    Float(Float),
    Integer(Integer),
    Bool(bool),
    Array(&'static [Expr]),

    /// A key value map, stored as slice in source code order.
    #[serde(serialize_with = "serialize_map")]
    Map(&'static [MapEntry]),
}

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum MapKey {
    Str(&'static str),
    Float(Float),
    Integer(Integer),
    Bool(bool),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MapEntry {
    pub key: MapKey,
    pub value: Expr,
}

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize)]
#[serde(untagged)]
pub enum Float {
    F32(f32),
    F64(f64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
#[serde(untagged)]
pub enum Integer {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    Usize(usize),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    Isize(isize),
}


impl fmt::Display for Float {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::F32(v) => v.fmt(f),
            Self::F64(v) => v.fmt(f),
        }
    }
}

impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::U8(i) => i.fmt(f),
            Self::U16(i) => i.fmt(f),
            Self::U32(i) => i.fmt(f),
            Self::U64(i) => i.fmt(f),
            Self::U128(i) => i.fmt(f),
            Self::Usize(i) => i.fmt(f),
            Self::I8(i) => i.fmt(f),
            Self::I16(i) => i.fmt(f),
            Self::I32(i) => i.fmt(f),
            Self::I64(i) => i.fmt(f),
            Self::I128(i) => i.fmt(f),
            Self::Isize(i) => i.fmt(f),
        }
    }
}

impl From<MapKey> for Expr {
    fn from(src: MapKey) -> Self {
        match src {
            MapKey::Str(v) => Self::Str(v),
            MapKey::Integer(v) => Self::Integer(v),
            MapKey::Float(v) => Self::Float(v),
            MapKey::Bool(v) => Self::Bool(v),
        }
    }
}

fn serialize_map<S>(map: &&'static [MapEntry], serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    use serde::ser::SerializeMap;

    let mut s = serializer.serialize_map(Some(map.len()))?;
    for entry in *map {
        s.serialize_entry(&entry.key, &entry.value)?;
    }
    s.end()
}
