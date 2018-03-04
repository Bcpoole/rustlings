// This script reads README-template.md and generates the playground links
// from the Rust source files in the various directories.

// To add a new exercise, add it to the appropriate place in README-template.md
// and then make sure to recompile this script (because the template gets
// included at compile time and then run it to generate a new version of
// README.md.

extern crate handlebars;
#[macro_use]
extern crate serde_json;

use handlebars::Handlebars;

use std::fs::File;
use std::io::prelude::*;

fn main() {
    let template = include_str!("../../README-template.hbs");
    let autogenerated_notice = "This file was autogenerated by the script in src/bin/generate_readme.rs.
Please edit either the script or the template in README-template.md in
order to make changes here rather than committing the changes directly.";

    let mut generated_readme = File::create("README.md").unwrap();

    let hbs = Handlebars::new();
    write!(
        generated_readme,
        "{}",
        hbs.render_template(
            template,
            &json!({ "autogenerated_notice": autogenerated_notice }),
        ).unwrap()
    ).unwrap();
}