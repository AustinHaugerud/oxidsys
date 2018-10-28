use language::operations::Operation;

pub struct AgentUnequipItemOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1774;

pub const IDENT: &str = "agent_unequip_item";

impl Operation for AgentUnequipItemOp {
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
