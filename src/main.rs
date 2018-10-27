extern crate clap;

#[macro_use]
extern crate nom;

mod commands;
mod compiler;
mod component;
mod language;

use clap::App;

fn main() {
    App::new("oxidsys")
        .version("1.0")
        .about("Oxidsys module system tool for M&B Warband")
        .author("Austin Jenkins")
        .get_matches();
}
