use language::operations::Operation;

pub struct TroopAddProficiencyPointsOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1525;

pub const IDENT: &str = "troop_add_proficiency_points";

impl Operation for TroopAddProficiencyPointsOp {
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
