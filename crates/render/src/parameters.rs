// Copyright (c) 2023 Sophie Katz
//
// This file is part of Sophie's Website.
//
// Sophie's Website is free software: you can redistribute it and/or modify it under the terms of
// the GNU General Public License as published by the Free Software Foundation, either version 3 of
// the License, or (at your option) any later version.
//
// Sophie's Website is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
// without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with Sophie's Website. If
// not, see <https://www.gnu.org/licenses/>.

//! Code to resolve parameter values.

use serde_json::json;
use std::{collections::HashMap, fs};

use crate::config::{Page, ParameterType, ParameterValue, Template};

/// Resolves a parameter value.
fn resolve_parameter(
    parameter_type: &ParameterType,
    parameter_value: &ParameterValue,
) -> serde_json::Value {
    match (parameter_type, parameter_value) {
        (ParameterType::String, ParameterValue::String(string_value)) => {
            serde_json::Value::String(string_value.clone())
        }
        (ParameterType::Loaded, ParameterValue::String(string_value)) => {
            serde_json::Value::String(fs::read_to_string(string_value).unwrap())
        }
        (ParameterType::Array(item_type), ParameterValue::Array(array_value)) => {
            serde_json::Value::Array(
                array_value
                    .iter()
                    .map(|item_value| {
                        resolve_parameter(item_type, &ParameterValue::String(item_value.clone()))
                    })
                    .collect(),
            )
        }
        _ => panic!("Invalid parameter type and value combination"),
    }
}

/// Resolves the parameters for a page.
pub fn resolve_parameters_for_page(template: &Template, page: &Page) -> serde_json::Value {
    serde_json::Value::Object(
        template
            .parameters
            .iter()
            .map(|(parameter_name, parameter)| {
                (
                    parameter_name.clone(),
                    resolve_parameter(&parameter.r#type, &page.parameters[parameter_name]),
                )
            })
            .collect(),
    )
}
