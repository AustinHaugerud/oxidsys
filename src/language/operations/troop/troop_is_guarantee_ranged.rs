use language::operations::Operation;

pub struct TroopIsGuaranteeRangedOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 153;

pub const IDENT: &str = "troop_is_guarantee_ranged";

impl Operation for TroopIsGuaranteeRangedOp {
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
