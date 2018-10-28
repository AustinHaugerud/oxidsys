use language::operations::Operation;

pub struct AgentClearRelationsWithAgentsOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1802;

pub const IDENT: &str = "agent_clear_relations_with_agents";

impl Operation for AgentClearRelationsWithAgentsOp {
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
