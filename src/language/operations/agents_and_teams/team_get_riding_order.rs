use language::operations::Operation;

pub struct TeamGetRidingOrderOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1786;

pub const IDENT: &str = "team_get_riding_order";

impl Operation for TeamGetRidingOrderOp {
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
