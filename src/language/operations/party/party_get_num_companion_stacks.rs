use language::operations::Operation;

pub struct PartyGetNumCompanionStacksOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1650;

pub const IDENT: &str = "party_get_num_companion_stacks";

impl Operation for PartyGetNumCompanionStacksOp {
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
