use language::operations::Operation;

pub struct PositionNormalizeOriginOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 741;

pub const IDENT: &str = "position_normalize_origin";

impl Operation for PositionNormalizeOriginOp {
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
