use language::operations::Operation;

pub struct AddXpToTroopOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1062;

pub const IDENT: &str = "add_xp_to_troop";

impl Operation for AddXpToTroopOp {
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
