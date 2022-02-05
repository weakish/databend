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

use std::fmt;
use std::marker::PhantomData;
use std::sync::Arc;

use common_datavalues2::NewColumn;
use common_datavalues2::StringColumn;
use common_datavalues2::StringType;
use common_exception::Result;
use uuid::Uuid;

use crate::scalars::function_factory::FunctionFeatures;
use crate::scalars::Function2;
use crate::scalars::Function2Description;

pub type UUIDv4Function = UUIDCreatorFunction<UUIDv4>;
pub type UUIDZeroFunction = UUIDCreatorFunction<UUIDZero>;

#[derive(Clone, Debug)]
pub struct UUIDCreatorFunction<T> {
    display_name: String,
    t: PhantomData<T>,
}

impl<T> UUIDCreatorFunction<T>
where T: UUIDCreator + Clone + Sync + Send + 'static
{
    pub fn try_create(display_name: &str) -> Result<Box<dyn Function2>> {
        Ok(Box::new(UUIDCreatorFunction::<T> {
            display_name: display_name.to_string(),
            t: PhantomData,
        }))
    }

    pub fn desc() -> Function2Description {
        Function2Description::creator(Box::new(Self::try_create))
            .features(FunctionFeatures::default())
    }
}

impl<T> fmt::Display for UUIDCreatorFunction<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}()", self.display_name)
    }
}

pub trait UUIDCreator {
    fn create() -> Uuid;
}

#[derive(Clone, Debug)]
pub struct UUIDv4;

impl UUIDCreator for UUIDv4 {
    fn create() -> Uuid {
        Uuid::new_v4()
    }
}

#[derive(Clone, Debug)]
pub struct UUIDZero;

impl UUIDCreator for UUIDZero {
    fn create() -> Uuid {
        Uuid::nil()
    }
}

impl<T> Function2 for UUIDCreatorFunction<T>
where T: UUIDCreator + Clone + Sync + Send + 'static
{
    fn name(&self) -> &str {
        self.display_name.as_str()
    }

    fn return_type(
        &self,
        _args: &[&common_datavalues2::DataTypePtr],
    ) -> Result<common_datavalues2::DataTypePtr> {
        Ok(StringType::arc())
    }

    fn eval(
        &self,
        _columns: &common_datavalues2::ColumnsWithField,
        _input_rows: usize,
    ) -> Result<common_datavalues2::ColumnRef> {
        let uuid = T::create();
        let col = StringColumn::new_from_slice(vec![uuid.to_string()]);

        Ok(Arc::new(col))
    }
}
