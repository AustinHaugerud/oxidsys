#![allow(dead_code)] // Temporary - We expect a lot of dead code for now, when completion is closer
                     // turn dead code warnings back on.

extern crate clap;

extern crate pest;
#[macro_use]
extern crate pest_derive;

#[macro_use]
extern crate lazy_static;

extern crate json;

mod commands;
mod compiler;
mod component;
mod documentation_service;
mod language;
mod loader;
mod module_creation;

use clap::App;
use clap::Arg;
use clap::SubCommand;

use documentation_service::DocumentationService;
use language::operations;

fn main() {
    let operations_map = operations::load_operands_map();

    let doc_service = DocumentationService::new(&operations_map);

    let matches = App::new("Oxidsys")
        .subcommand(
            SubCommand::with_name("documentation")
                .about("Get documentation for operand.")
                .arg(
                    Arg::with_name("operation")
                        .help("Operation to print documentation for")
                        .required(true),
                ),
        ).subcommand(
            SubCommand::with_name("new")
                .about("Create new module")
                .arg(Arg::with_name("name").required(true)),
        ).subcommand(
        SubCommand::with_name("build")
            .about("Build a target.")
            .arg(Arg::with_name("target").required(true))
        )
        .get_matches();

    if let Some(documentation_matches) = matches.subcommand_matches("documentation") {
        let op_ident = documentation_matches
            .value_of("operation")
            .expect("Bug: No operation identifier when expected.");
        println!("{}", doc_service.request_documentation(op_ident));
    }

    if let Some(new_mod_matches) = matches.subcommand_matches("new") {
        let name = new_mod_matches
            .value_of("name")
            .expect("Bug: No name given when expected.");
        if module_creation::blank_module::init_blank_module(name).is_err() {
            println!("Failed to create new module.");
        }
    }

    if let Some(build_matches) = matches.subcommand_matches("build") {
        let target = build_matches.value_of("target").expect("Bug: No build target when expected.");
        if let Err(e) = commands::build::execute_compiler(target) {
            println!("Build failed: {}", e);
        }
        else {
            println!("Build success.");
        }
    }
}
