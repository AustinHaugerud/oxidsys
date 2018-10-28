use language::operations::Operation;

pub struct PartyGetAttachedToOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1694;

pub const IDENT: &str = "party_get_attached_to";

impl Operation for PartyGetAttachedToOp {
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
