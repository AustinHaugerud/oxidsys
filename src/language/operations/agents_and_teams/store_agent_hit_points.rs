use language::operations::Operation;

pub struct StoreAgentHitPointsOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1720;

pub const IDENT: &str = "store_agent_hit_points";

impl Operation for StoreAgentHitPointsOp {
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
