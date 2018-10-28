use language::operations::Operation;

pub struct AgentSetAttachedScenePropYOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1809;

pub const IDENT: &str = "agent_set_attached_scene_prop_y";

impl Operation for AgentSetAttachedScenePropYOp {
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
