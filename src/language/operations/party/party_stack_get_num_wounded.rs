use language::operations::Operation;

pub struct PartyStackGetNumWoundedOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1654;

pub const IDENT: &str = "party_stack_get_num_wounded";

impl Operation for PartyStackGetNumWoundedOp {
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
