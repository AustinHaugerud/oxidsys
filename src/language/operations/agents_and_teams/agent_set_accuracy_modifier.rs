use language::operations::Operation;

pub struct AgentSetAccuracyModifierOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2092;

pub const IDENT: &str = "agent_set_accuracy_modifier";

impl Operation for AgentSetAccuracyModifierOp {
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
