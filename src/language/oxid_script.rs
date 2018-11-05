use std::path;
use std::fs;
use language::oxid_parser::*;
use language::registers::{NREGISTERS, PREGISTERS, SREGISTERS};
use std::collections::HashSet;

pub struct OperandCall {
    op_id : String,
    params : Vec<String>,
}

pub struct Script {
    op_calls : Vec<OperandCall>,
}

impl Script {
    pub fn from_file(path : &path::PathBuf) -> Result<Script,String> {

        let content = fs::read_to_string(path).map_err(|_| format!("Failed to load script content: {:?}", path))?;

        let statements = parse_script(&content)?;

        //let mut operand_calls = vec![];
        let mut errors = vec![];

        let mut var_registry : HashSet<String> = HashSet::new();

        let mut param_registry : HashSet<String> = HashSet::new();

        let mut nreg_registry : HashSet<String> = HashSet::new();
        let mut sreg_registry : HashSet<String> = HashSet::new();
        let mut preg_registry : HashSet<String> = HashSet::new();

        for statement in statements.iter() {
            match statement {
                CollectedStatement::RegisterDeclaration(decl) => {
                    match decl.get_kind() {
                        RegisterKind::Numeric => {
                            if NREGISTERS.contains_key(decl.get_reg_id()) {
                                if nreg_registry.contains(decl.get_reg_id()) {
                                    errors.push(format!("Duplicate register declaration {}.", decl.get_reg_id()));
                                }
                                else {
                                    nreg_registry.insert(decl.get_reg_id().to_owned());
                                }
                            }
                            else {
                                errors.push(format!("Invalid register identifier {}.", decl.get_reg_id()));
                            }
                        },
                        RegisterKind::Position => {
                            if PREGISTERS.contains_key(decl.get_reg_id()) {
                                if preg_registry.contains(decl.get_reg_id()) {
                                    errors.push(format!("Duplicate pregister declaration {}.", decl.get_reg_id()));
                                }
                                else {
                                    preg_registry.insert(decl.get_reg_id().to_owned());
                                }
                            }
                            else {
                                errors.push(format!("Invalid pregister identifier {}.", decl.get_reg_id()));
                            }
                        },
                        RegisterKind::String => {
                            if SREGISTERS.contains_key(decl.get_reg_id()) {
                                if sreg_registry.contains(decl.get_reg_id()) {
                                    errors.push(format!("Duplicate sregister declaration {}.", decl.get_reg_id()));
                                }
                                else {
                                    sreg_registry.insert(decl.get_reg_id().to_owned());
                                }
                            }
                            else {
                                errors.push(format!("Invalid sregister identifier {}.", decl.get_reg_id()));
                            }
                        },
                    }
                },

                CollectedStatement::ParamDeclaration(decl) => {

                },

                CollectedStatement::VariableDeclaration(decl) => {

                },

                CollectedStatement::VariableChange(varc) => {},

                CollectedStatement::OperandCall(call) => {

                }
            }
        }

        Err("".to_owned())
    }
}
