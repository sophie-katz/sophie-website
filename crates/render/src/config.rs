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

//! Configuration model for the site index.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// The configuration model for the site index.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    /// The templates used to render the site.
    pub templates: HashMap<String, Template>,

    /// The actual pages that instantiate those templates.
    pub pages: Vec<Page>,
}

/// A template used to render the site.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Template {
    /// The path to the template.
    pub path: String,

    /// The parameters used to configure the template.
    pub parameters: HashMap<String, Parameter>,
}

/// A parameter used to configure a template.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Parameter {
    /// The type of the parameter.
    #[serde(rename = "type")]
    pub r#type: ParameterType,
}

/// The type of a parameter.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ParameterType {
    /// A simple string to be injected.
    #[serde(rename = "string")]
    String,
    /// A path to a file to be resolved, and then loaded as a string.
    #[serde(rename = "loaded")]
    Loaded,
    /// An array of values of any of the other types.
    #[serde(rename = "array")]
    Array(Box<ParameterType>),
}

/// A page that instantiates a template.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Page {
    /// The path to the page
    pub path: String,
    /// The template to instantiate.
    pub template: String,
    /// The parameters to pass to the template.
    pub parameters: HashMap<String, ParameterValue>,
}

/// A parameter value that can be either a string or an array of strings.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ParameterValue {
    /// As a single string.
    String(String),
    /// As an array of strings.
    Array(Vec<String>),
}
