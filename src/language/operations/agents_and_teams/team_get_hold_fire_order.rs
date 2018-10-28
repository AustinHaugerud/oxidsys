use language::operations::Operation;

pub struct TeamGetHoldFireOrderOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1784;

pub const IDENT: &str = "team_get_hold_fire_order";

impl Operation for TeamGetHoldFireOrderOp {
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
