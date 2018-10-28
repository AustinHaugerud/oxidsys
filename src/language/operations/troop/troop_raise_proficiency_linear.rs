use language::operations::Operation;

pub struct TroopRaiseProficiencyLinearOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1523;

pub const IDENT: &str = "troop_raise_proficiency_linear";

impl Operation for TroopRaiseProficiencyLinearOp {
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
