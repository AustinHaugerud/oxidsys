use language::operations::Operation;

pub struct GetPlayerAgentNoOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1700;

pub const IDENT: &str = "get_player_agent_no";

impl Operation for GetPlayerAgentNoOp {
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
