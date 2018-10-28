use language::operations::Operation;

pub struct PartyStackGetSizeOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1653;

pub const IDENT: &str = "party_stack_get_size";

impl Operation for PartyStackGetSizeOp {
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
