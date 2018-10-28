use language::operations::Operation;

pub struct StartMultiplayerMissionOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 470;

pub const IDENT: &str = "start_multiplayer_mission";

impl Operation for StartMultiplayerMissionOp {
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
