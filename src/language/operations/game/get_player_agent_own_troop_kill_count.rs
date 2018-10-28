use language::operations::Operation;

pub struct GetPlayerAgentOwnTroopKillCountOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1705;

pub const IDENT: &str = "get_player_agent_own_troop_kill_count";

impl Operation for GetPlayerAgentOwnTroopKillCountOp {
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
