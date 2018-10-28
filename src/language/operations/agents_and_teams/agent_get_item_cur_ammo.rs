use language::operations::Operation;

pub struct AgentGetItemCurAmmoOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1977;

pub const IDENT: &str = "agent_get_item_cur_ammo";

impl Operation for AgentGetItemCurAmmoOp {
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
