use language::operations::Operation;

pub struct SetQuestProgressionOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1285;

pub const IDENT: &str = "set_quest_progression";

impl Operation for SetQuestProgressionOp {
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
