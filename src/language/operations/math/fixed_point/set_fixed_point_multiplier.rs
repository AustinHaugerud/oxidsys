use language::operations::Operation;

pub struct SetFixedPointMultiplierOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2124;

pub const IDENT: &str = "set_fixed_point_multiplier";

impl Operation for SetFixedPointMultiplierOp {
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
