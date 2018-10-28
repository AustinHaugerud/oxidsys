use language::operations::Operation;

pub struct TroopRaiseProficiencyOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1522;

pub const IDENT: &str = "troop_raise_proficiency";

impl Operation for TroopRaiseProficiencyOp {
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
