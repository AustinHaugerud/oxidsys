use language::operations::Operation;

pub struct ChangeScreenQuitOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2055;

pub const IDENT: &str = "change_screen_quit";

impl Operation for ChangeScreenQuitOp {
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
