use language::operations::Operation;

pub struct AgentGetActionDirOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1767;

pub const IDENT: &str = "agent_get_action_dir";

impl Operation for AgentGetActionDirOp {
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
