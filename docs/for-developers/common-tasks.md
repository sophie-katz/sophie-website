<!--
Copyright (c) 2023 Sophie Katz

This file is part of Sophie's Website.

Sophie's Website is free software: you can redistribute it and/or modify it under the terms of the
GNU General Public License as published by the Free Software Foundation, either version 3 of the
License, or (at your option) any later version.

Sophie's Website is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
GNU General Public License for more details.
 
You should have received a copy of the GNU General Public License along with Sophie's Website. If
not, see <https://www.gnu.org/licenses/>.
-->

# Common tasks

A guide for how to do common developer tasks around the codebase. See [Cargo Commands](https://doc.rust-lang.org/cargo/commands/) for most standard tasks.

**NOTE:** Clippy is used for linting instead of `cargo check`.

## Code coverage

Run `cargo cmd coverage:lcov` to generate code coverage data that can be used in VS Code for coverage gutters. Run `cargo cmd coverage:html` to generate an HTML report.

Both of these commands rely on having [Cargo Commander](https://github.com/simonhyll/cargo-commander) installed.
