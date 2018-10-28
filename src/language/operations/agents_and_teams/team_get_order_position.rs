use language::operations::Operation;

pub struct TeamGetOrderPositionOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1794;

pub const IDENT: &str = "team_get_order_position";

impl Operation for TeamGetOrderPositionOp {
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
