use language::operations::Operation;

pub struct AgentSetAttachedScenePropXOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1758;

pub const IDENT: &str = "agent_set_attached_scene_prop_x";

impl Operation for AgentSetAttachedScenePropXOp {
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
