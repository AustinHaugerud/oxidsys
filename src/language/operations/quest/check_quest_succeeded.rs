use language::operations::Operation;

pub struct CheckQuestSucceededOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 202;

pub const IDENT: &str = "check_quest_succeeded";

impl Operation for CheckQuestSucceededOp {
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
