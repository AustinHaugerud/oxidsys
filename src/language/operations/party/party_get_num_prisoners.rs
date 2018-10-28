use language::operations::Operation;

pub struct PartyGetNumPrisonersOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1602;

pub const IDENT: &str = "party_get_num_prisoners";

impl Operation for PartyGetNumPrisonersOp {
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
