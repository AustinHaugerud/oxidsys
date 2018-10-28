use language::operations::Operation;

pub struct AgentAddRelationWithAgentOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1803;

pub const IDENT: &str = "agent_add_relation_with_agent";

impl Operation for AgentAddRelationWithAgentOp {
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
