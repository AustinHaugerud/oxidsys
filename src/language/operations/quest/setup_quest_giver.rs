use language::operations::Operation;

pub struct SetupQuestGiverOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1291;

pub const IDENT: &str = "setup_quest_giver";

impl Operation for SetupQuestGiverOp {
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
