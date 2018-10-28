use language::operations::Operation;

pub struct TutorialMessageSetCenterJustifyOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1125;

pub const IDENT: &str = "tutorial_message_set_center_justify";

impl Operation for TutorialMessageSetCenterJustifyOp {
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
