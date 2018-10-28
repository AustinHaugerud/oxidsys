use language::operations::Operation;

pub struct GetPlayerAgentKillCountOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1701;

pub const IDENT: &str = "get_player_agent_kill_count";

impl Operation for GetPlayerAgentKillCountOp {
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
