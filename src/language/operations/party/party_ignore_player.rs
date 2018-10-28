use language::operations::Operation;

pub struct PartyIgnorePlayerOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1644;

pub const IDENT: &str = "party_ignore_player";

impl Operation for PartyIgnorePlayerOp {
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
