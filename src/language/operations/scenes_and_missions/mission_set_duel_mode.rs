use language::operations::Operation;

pub struct MissionSetDuelModeOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2006;

pub const IDENT: &str = "mission_set_duel_mode";

impl Operation for MissionSetDuelModeOp {
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
