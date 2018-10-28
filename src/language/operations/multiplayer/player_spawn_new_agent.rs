use language::operations::Operation;

pub struct PlayerSpawnNewAgentOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 409;

pub const IDENT: &str = "player_spawn_new_agent";

impl Operation for PlayerSpawnNewAgentOp {
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
