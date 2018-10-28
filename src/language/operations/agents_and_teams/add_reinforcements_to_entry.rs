use language::operations::Operation;

pub struct AddReinforcementsToEntryOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1930;

pub const IDENT: &str = "add_reinforcements_to_entry";

impl Operation for AddReinforcementsToEntryOp {
    fn op_code(&self) -> u16 {
        OP_CODE
    }

    fn documentation(&self) -> &'static str {
        DOC
    }

    fn identifier(&self) -> &'static str {
        IDENT
    }
}
