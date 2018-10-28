use language::operations::Operation;

pub struct PartySetExtraIconOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1682;

pub const IDENT: &str = "party_set_extra_icon";

impl Operation for PartySetExtraIconOp {
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
