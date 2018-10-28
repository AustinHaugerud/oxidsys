#![allow(dead_code)] // Temporary - We expect a lot of dead code for now, when completion is closer
                     // turn dead code warnings back on.

extern crate clap;

#[macro_use]
extern crate nom;

mod compiler;
mod component;
mod documentation_service;
mod language;
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
        ).get_matches();

    if let Some(documentation_matches) = matches.subcommand_matches("documentation") {
        let op_ident = documentation_matches
            .value_of("operation")
            .expect("Bug: No operation identifier when expected.");
        println!("{}", doc_service.request_documentation(op_ident));
    }
}
