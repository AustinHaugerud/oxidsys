use language::operations::Operation;

pub struct TryForAgentsOp;

const DOC: &str = r#"
Iterate over all agents in the scene.
Format: try_for_agents <iterator>;
"#;

pub const OP_CODE: u16 = 12;

pub const IDENT: &str = "try_for_agents";

impl Operation for TryForAgentsOp {
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
