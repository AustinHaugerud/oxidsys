use language::operations::Operation;

pub struct PartySetAggressivenessOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1606;

pub const IDENT: &str = "party_set_aggressiveness";

impl Operation for PartySetAggressivenessOp {
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
