use language::operations::Operation;

pub struct SetJumpEntryOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1912;

pub const IDENT: &str = "set_jump_entry";

impl Operation for SetJumpEntryOp {
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
