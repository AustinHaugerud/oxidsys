use language::operations::Operation;

pub struct PositionCopyOriginOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 719;

pub const IDENT: &str = "position_copy_origin";

impl Operation for PositionCopyOriginOp {
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
