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

//! Code to load templates.

use handlebars::Handlebars;

use crate::config::Template;

pub fn load_template_into_handlebars(
    handlebars: &mut Handlebars,
    template_name: &str,
    template: &Template,
) {
    handlebars
        .register_template_file(template_name, &template.path)
        .unwrap();
}
