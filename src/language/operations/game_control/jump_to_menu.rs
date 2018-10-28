use language::operations::Operation;

pub struct JumpToMenuOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2060;

pub const IDENT: &str = "jump_to_menu";

impl Operation for JumpToMenuOp {
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
