use language::operations::Operation;

pub struct AgentGetBonePositionOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2076;

pub const IDENT: &str = "agent_get_bone_position";

impl Operation for AgentGetBonePositionOp {
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
