use language::operations::Operation;

pub struct AgentSetLookTargetPositionOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1744;

pub const IDENT: &str = "agent_set_look_target_position";

impl Operation for AgentSetLookTargetPositionOp {
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
