use language::operations::Operation;

pub struct AgentSetMaxHitPointsOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2090;

pub const IDENT: &str = "agent_set_max_hit_points";

impl Operation for AgentSetMaxHitPointsOp {
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
