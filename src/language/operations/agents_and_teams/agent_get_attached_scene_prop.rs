use language::operations::Operation;

pub struct AgentGetAttachedScenePropOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1756;

pub const IDENT: &str = "agent_get_attached_scene_prop";

impl Operation for AgentGetAttachedScenePropOp {
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
