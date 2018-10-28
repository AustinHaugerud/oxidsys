use language::operations::Operation;

pub struct TeamSetOrderListenerOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1795;

pub const IDENT: &str = "team_set_order_listener";

impl Operation for TeamSetOrderListenerOp {
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
