//! Module containing extension implementations for the types auto-generated by `quicktype` in `types.rs`.
//! The implementations are included here because:
//! 1. `quicktype` does not support these additional traits
//! 2. Manual changes made in `types.rs` will be overwritten by future calls to `quicktype.sh`.
#![allow(clippy::derivable_impls)]
use crate::PipesContextData;

use crate::{Method, PipesMessage};
use std::collections::HashMap;

impl Default for PipesContextData {
    fn default() -> Self {
        Self {
            asset_keys: None,
            code_version_by_asset_key: None,
            extras: None,
            job_name: None,
            partition_key: None,
            partition_key_range: None,
            partition_time_window: None,
            provenance_by_asset_key: None,
            retry_number: 0,
            run_id: String::new(),
        }
    }
}

impl PipesMessage {
    pub fn new(method: Method, params: Option<HashMap<String, Option<serde_json::Value>>>) -> Self {
        Self {
            dagster_pipes_version: "0.1".to_string(), // TODO: Make `const`
            method,
            params,
        }
    }
}
