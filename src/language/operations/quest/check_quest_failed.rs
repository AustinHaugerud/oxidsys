use language::operations::Operation;

pub struct CheckQuestFailedOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 203;

pub const IDENT: &str = "check_quest_failed";

impl Operation for CheckQuestFailedOp {
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
