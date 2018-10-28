use language::operations::Operation;

pub struct TutorialMessageSetSizeOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1124;

pub const IDENT: &str = "tutorial_message_set_size";

impl Operation for TutorialMessageSetSizeOp {
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
