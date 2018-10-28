use language::operations::Operation;

pub struct AgentSetHitPointsOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1721;

pub const IDENT: &str = "agent_set_hit_points";

impl Operation for AgentSetHitPointsOp {
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
