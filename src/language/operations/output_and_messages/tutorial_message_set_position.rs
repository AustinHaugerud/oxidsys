use language::operations::Operation;

pub struct TutorialMessageSetPositionOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1123;

pub const IDENT: &str = "tutorial_message_set_position";

impl Operation for TutorialMessageSetPositionOp {
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
