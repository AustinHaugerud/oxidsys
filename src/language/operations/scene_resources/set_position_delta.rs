use language::operations::Operation;

pub struct SetPositionDeltaOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1955;

pub const IDENT: &str = "set_position_delta";

impl Operation for SetPositionDeltaOp {
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
