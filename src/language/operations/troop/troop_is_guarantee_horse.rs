use language::operations::Operation;

pub struct TroopIsGuaranteeHorseOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 154;

pub const IDENT: &str = "troop_is_guarantee_horse";

impl Operation for TroopIsGuaranteeHorseOp {
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
