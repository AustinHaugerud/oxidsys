use language::operations::Operation;

pub struct AgentSetAttachedScenePropOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1757;

pub const IDENT: &str = "agent_set_attached_scene_prop";

impl Operation for AgentSetAttachedScenePropOp {
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
