use language::operations::Operation;

pub struct PartySetExtraTextOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1605;

pub const IDENT: &str = "party_set_extra_text";

impl Operation for PartySetExtraTextOp {
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
