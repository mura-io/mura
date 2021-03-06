// Copyright 2020 Sujith Jay Nair
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

use arrow::datatypes::Schema;
use std::collections::HashMap;
use std::sync::Arc;

/// SchemaCatalog provides metadata about tables referenced in SQL statements
pub trait SchemaCatalog {
    fn fetch_table_info(&self, name: &str) -> Option<Arc<Schema>>;
}

pub struct DummySchemaCatalog {
    pub datasource: HashMap<String, Option<Arc<Schema>>>,
}

impl SchemaCatalog for DummySchemaCatalog{
    fn fetch_table_info(&self, name: &str) -> Option<Arc<Schema>> {
        self.datasource.get(name).unwrap().clone()
    }
}