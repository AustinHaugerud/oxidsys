use language::operations::Operation;

pub struct TutorialMessageSetBackgroundOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1126;

pub const IDENT: &str = "tutorial_message_set_background";

impl Operation for TutorialMessageSetBackgroundOp {
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
