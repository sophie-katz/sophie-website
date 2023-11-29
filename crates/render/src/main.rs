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

//! A simple site renderer.

mod config;
mod parameters;
mod render;
mod templates;

use config::Config;

use std::{
    fs::{self, File},
    io::BufReader,
};

use clap::Parser;

/// The command line arguments.
#[derive(Parser, Debug)]
struct CommandLineArgs {
    /// The path to the configuration file.
    #[arg(short, long)]
    config: String,
    /// The distribution file path.
    #[arg(short, long)]
    dist: String,
    /// The path to the static files.
    #[arg(short, long)]
    r#static: String,
}

fn main() {
    let args = CommandLineArgs::parse();

    let file = File::open(args.config).unwrap();
    let reader = BufReader::new(file);
    let config: Config = serde_yaml::from_reader(reader).unwrap();

    render::render(&config, &args.dist);

    let mut options = fs_extra::dir::CopyOptions::new();

    options.overwrite = true;

    let items: Vec<String> = fs::read_dir(args.r#static)
        .unwrap()
        .map(|entry| entry.unwrap().path().to_str().unwrap().to_owned())
        .collect();

    fs_extra::copy_items(items.as_ref(), args.dist, &options).unwrap();
}
