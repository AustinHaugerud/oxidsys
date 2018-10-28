use language::operations::Operation;

pub struct ChangeScreenViewCharacterOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2046;

pub const IDENT: &str = "change_screen_view_character";

impl Operation for ChangeScreenViewCharacterOp {
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
