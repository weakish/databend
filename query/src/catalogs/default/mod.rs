// Copyright 2022 Datafuse Labs.
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

mod backends;
mod catalog_context;
mod database_catalog;
mod immutable_catalog;
mod mutable_catalog;
pub mod table_id_ranges;
pub mod table_memory_meta;

pub use database_catalog::DatabaseCatalog;
// for "unit" test
pub use immutable_catalog::ImmutableCatalog;
pub use mutable_catalog::MutableCatalog;
