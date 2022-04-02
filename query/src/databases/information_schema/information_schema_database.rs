//  Copyright 2022 Datafuse Labs.
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.

use std::sync::Arc;

use common_meta_types::DatabaseInfo;
use common_meta_types::DatabaseMeta;

use crate::catalogs::InMemoryMetas;
use crate::databases::Database;
use crate::storages::information_schema::ColumnsTable;
use crate::storages::Table;

#[derive(Clone)]
pub struct InformationSchemaDatabase {
    db_info: DatabaseInfo,
}

impl InformationSchemaDatabase {
    pub fn create(sys_db_meta: &mut InMemoryMetas) -> Self {
        // todo(veeupup): create needed tables for infomation schema
        // register sys_db_meta
        let table_list: Vec<Arc<dyn Table>> = vec![ColumnsTable::create(sys_db_meta.next_id())];

        for tbl in table_list.into_iter() {
            sys_db_meta.insert("information_schema", tbl);
        }

        let db_info = DatabaseInfo {
            database_id: 1,
            db: "information_schema".to_string(),
            meta: DatabaseMeta {
                engine: "SYSTEM".to_string(),
                ..Default::default()
            },
        };

        Self { db_info }
    }
}

#[async_trait::async_trait]
impl Database for InformationSchemaDatabase {
    fn name(&self) -> &str {
        "information_schema"
    }

    fn get_db_info(&self) -> &DatabaseInfo {
        &self.db_info
    }
}
