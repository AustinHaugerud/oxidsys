use language::operations::Operation;

pub struct TeamGetMovementOrderOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1785;

pub const IDENT: &str = "team_get_movement_order";

impl Operation for TeamGetMovementOrderOp {
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
