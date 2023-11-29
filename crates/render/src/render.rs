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

//! Page renderer

use handlebars::Handlebars;

use crate::{
    config::{Config, Page, Template},
    parameters::resolve_parameters_for_page,
};

/// Renders a page to a string.
fn render_page_to_string(handlebars: &Handlebars, config: &Config, page: &Page) -> String {
    let template = &config.templates[&page.template];

    let parameters = resolve_parameters_for_page(template, page);

    handlebars.render(&page.template, &parameters).unwrap()
}

/// Renders the site.
pub fn render(config: &Config, dist_path: &str) {
    let mut handlebars = Handlebars::new();

    handlebars.register_escape_fn(handlebars::no_escape);

    for (template_name, template) in &config.templates {
        crate::templates::load_template_into_handlebars(&mut handlebars, template_name, template);
    }

    for page in &config.pages {
        let rendered_page = render_page_to_string(&handlebars, config, page);

        let path = format!("{}/{}", dist_path, page.path);

        std::fs::create_dir_all(std::path::Path::new(&path).parent().unwrap()).unwrap();

        std::fs::write(path, rendered_page).unwrap();
    }
}
