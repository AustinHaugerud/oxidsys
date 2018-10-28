use language::operations::Operation;

pub struct SetupQuestTextOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1290;

pub const IDENT: &str = "setup_quest_text";

impl Operation for SetupQuestTextOp {
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
