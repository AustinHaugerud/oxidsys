use language::operations::Operation;

pub struct CheckQuestConcludedOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 204;

pub const IDENT: &str = "check_quest_concluded";

impl Operation for CheckQuestConcludedOp {
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
