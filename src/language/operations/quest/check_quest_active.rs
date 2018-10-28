use language::operations::Operation;

pub struct CheckQuestActiveOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 200;

pub const IDENT: &str = "check_quest_active";

impl Operation for CheckQuestActiveOp {
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
