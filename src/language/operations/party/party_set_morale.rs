use language::operations::Operation;

pub struct PartySetMoraleOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1672;

pub const IDENT: &str = "party_set_morale";

impl Operation for PartySetMoraleOp {
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
