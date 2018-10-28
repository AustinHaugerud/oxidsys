use language::operations::Operation;

pub struct StartMissionConversationOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1920;

pub const IDENT: &str = "start_mission_conversation";

impl Operation for StartMissionConversationOp {
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
