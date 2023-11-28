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

use std::fs;

use crate::config::{ParameterType, ParameterValue};

/// Resolves a parameter value.
pub fn resolve_parameter(
    parameter_type: &ParameterType,
    parameter_value: &ParameterValue,
) -> Vec<String> {
    match (parameter_type, parameter_value) {
        (ParameterType::String, ParameterValue::String(string_value)) => vec![string_value.clone()],
        (ParameterType::Loaded, ParameterValue::String(string_value)) => {
            vec![fs::read_to_string(string_value).unwrap()]
        }
        (ParameterType::Array(item_type), ParameterValue::Array(array_value)) => array_value
            .iter()
            .flat_map(|item_value| {
                resolve_parameter(item_type, &ParameterValue::String(item_value.clone()))
            })
            .collect(),
        _ => panic!("Invalid parameter type and value combination"),
    }
}
