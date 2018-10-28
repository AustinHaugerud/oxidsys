use language::operations::Operation;

pub struct ScenePropHasAgentOnItOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1801;

pub const IDENT: &str = "scene_prop_has_agent_on_it";

impl Operation for ScenePropHasAgentOnItOp {
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
