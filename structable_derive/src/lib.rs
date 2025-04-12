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
//
// SPDX-License-Identifier: Apache-2.0

//! Most likely you do not want to use this crate directly. It is a helper for the OpenStack
//! 
//! This crate implements derive macros for converting structures
//! (or structure vectors) as tables (vector of vector of strings -
//! as rows and columns).
//! 
//! ```rust
//! # use std::collections::BTreeSet;
//! # use serde_json::Value;
//! # use serde::Serialize;
//! # use structable_derive::StructTable;
//! # 
//! # pub trait StructTable {
//! #     fn headers<O: StructTableOptions>(config: &O) -> Vec<String>;
//! #     fn data<O: StructTableOptions>(&self, config: &O) -> Vec<Option<String>>;
//! #     fn status(&self) -> Option<String>;
//! # }
//! # 
//! # #[derive(Clone, Debug, Default)]
//! # pub struct OutputConfig {
//! #     pub fields: BTreeSet<String>,
//! #     pub wide: bool,
//! #     pub pretty: bool,
//! # }
//! # 
//! # pub trait StructTableOptions {
//! #     fn wide_mode(&self) -> bool;
//! #     fn pretty_mode(&self) -> bool;
//! #     fn should_return_field<S: AsRef<str>>(&self, field: S, is_wide_field: bool) -> bool;
//! # }
//! # 
//! # impl StructTableOptions for OutputConfig {
//! #     fn wide_mode(&self) -> bool {
//! #         self.wide
//! #     }
//! # 
//! #     fn pretty_mode(&self) -> bool {
//! #         self.pretty
//! #     }
//! # 
//! #     fn should_return_field<S: AsRef<str>>(&self, field: S, is_wide_field: bool) -> bool {
//! #         if !is_wide_field {
//! #             self.fields.is_empty() || self.fields.contains(field.as_ref())
//! #         } else {
//! #             (self.fields.is_empty() && self.wide_mode()) || self.fields.contains(field.as_ref())
//! #         }
//! #     }
//! # }
//! # 
//! #[derive(Serialize, StructTable)]
//! struct User {
//!     #[structable(title = "ID")]
//!     id: u64,
//!     first_name: &'static str,
//!     last_name: &'static str,
//!     #[structable(title = "Long(only in wide mode)", wide)]
//!     extra: &'static str,
//!     #[structable(optional, pretty)]
//!     complex_data: Option<Value>
//! }
//! ```
mod structable;

use darling::FromDeriveInput;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(StructTable, attributes(structable))]
/// Derive macro to implementing `VecTable` traits
pub fn openstack_result_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    let receiver = structable::TableStructInputReceiver::from_derive_input(&input).unwrap();
    let tokens = quote!(#receiver);
    tokens.into()
}
