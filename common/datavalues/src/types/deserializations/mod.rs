// Copyright 2021 Datafuse Labs.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use common_exception::Result;
use common_io::prelude::CpBufferReader;
use enum_dispatch::enum_dispatch;
use serde_json::Value;

use crate::prelude::*;

mod boolean;
mod date;
mod date_time;
mod null;
mod nullable;
mod number;
mod string;
mod variant;

pub use boolean::*;
pub use date::*;
pub use date_time::*;
pub use null::*;
pub use nullable::*;
pub use number::*;
pub use string::*;
pub use variant::*;

#[enum_dispatch]
pub trait TypeDeserializer: Send + Sync {
    fn de_binary(&mut self, reader: &mut &[u8]) -> Result<()>;

    fn de_default(&mut self);

    fn de_fixed_binary_batch(&mut self, reader: &[u8], step: usize, rows: usize) -> Result<()>;

    fn de_json(&mut self, reader: &Value) -> Result<()>;

    fn de_null(&mut self) -> bool {
        false
    }

    fn de_whole_text(&mut self, reader: &[u8]) -> Result<()>;

    fn de_text(&mut self, reader: &mut CpBufferReader) -> Result<()>;

    fn de_text_csv(&mut self, reader: &mut CpBufferReader) -> Result<()> {
        self.de_text(reader)
    }

    fn de_text_json(&mut self, reader: &mut CpBufferReader) -> Result<()> {
        self.de_text(reader)
    }

    fn de_text_quoted(&mut self, reader: &mut CpBufferReader) -> Result<()> {
        self.de_text(reader)
    }

    fn append_data_value(&mut self, value: DataValue) -> Result<()>;

    /// Note this method will return err only when inner builder is empty.
    fn pop_data_value(&mut self) -> Result<DataValue>;

    fn finish_to_column(&mut self) -> ColumnRef;
}

#[enum_dispatch(TypeDeserializer)]
pub enum TypeDeserializerImpl {
    Null(NullDeserializer),
    Nullable(NullableDeserializer),
    Boolean(BooleanDeserializer),
    Int8(NumberDeserializer<i8>),
    Int16(NumberDeserializer<i16>),
    Int32(NumberDeserializer<i32>),
    Int64(NumberDeserializer<i64>),
    UInt8(NumberDeserializer<u8>),
    UInt16(NumberDeserializer<u16>),
    UInt32(NumberDeserializer<u32>),
    UInt64(NumberDeserializer<u64>),
    Float32(NumberDeserializer<f32>),
    Float64(NumberDeserializer<f64>),

    Date(DateDeserializer<i32>),
    Interval(DateDeserializer<i64>),
    DateTime(DateTimeDeserializer<i64>),
    String(StringDeserializer),
    // TODO
    // Array(ArrayDeserializer),
    // Struct(StructDeserializer),
    Variant(VariantDeserializer),
}
