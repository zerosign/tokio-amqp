use std::slice::SliceIndex;
use std::collections::HashMap;
use std::convert::{TryInto, TryFrom};
use bytes::Bytes;

#[derive(Debug, Clone, PartialEq)]
pub enum TableEntry {
    Bool(bool),
    ShortShortInt(i8),
    ShortShortUint(u8),
    ShortInt(i16),
    ShortUint(u16),
    LongInt(i32),
    LongUint(u32),
    LongLongInt(i64),
    LongLongUint(u64),
    Float(f32),
    Double(f64),
    DecimalValue(u8, u32),
    // ShortString(String),
    LongString(String),
    FieldArray(Vec<TableEntry>),
    Timestamp(u64),
    FieldTable(Table),
    Void
}

pub type Table = HashMap<String, TableEntry>;

// decode table entry enums from bytes
// impl TryFrom<T: SliceIndex + 'static> for TableEntry {
//     fn try_from() -> Result<TableEntry, ()> -> {

//     }
// }


// encode table entry enum to bytes
// impl TryInto<T:
