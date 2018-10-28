use language::operations::Operation;

pub struct SetJumpMissionOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1911;

pub const IDENT: &str = "set_jump_mission";

impl Operation for SetJumpMissionOp {
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
