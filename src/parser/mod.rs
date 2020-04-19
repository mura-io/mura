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

use anyhow::Result;
use sqlparser::ast::Statement;
use sqlparser::dialect::PostgreSqlDialect;
use sqlparser::parser::Parser;

#[derive(Debug, Clone)]
pub enum FileType {
    /// Apache Parquet
    Parquet,
}

/// A sample query is: 
///     CREATE EXTERNAL TABLE userdata STORED AS PARQUET LOCATION 
///     'mura/src/examples/userdata1.parquet';
pub fn parse(query: String) -> Result<Vec<Statement>> {
    let dialect = PostgreSqlDialect {};
    return Parser::parse_sql(&dialect, query).map_err(|e| e.into());
}