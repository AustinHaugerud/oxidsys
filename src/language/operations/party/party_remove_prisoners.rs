use language::operations::Operation;

pub struct PartyRemovePrisonersOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1616;

pub const IDENT: &str = "party_remove_prisoners";

impl Operation for PartyRemovePrisonersOp {
    fn op_code(&self) -> u32 {
        OP_CODE
    }

    fn documentation(&self) -> &'static str {
        DOC
    }

    fn identifier(&self) -> &'static str {
        IDENT
    }
}
