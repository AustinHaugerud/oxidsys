use language::operations::Operation;

pub struct PositionIsBehindPositionOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 714;

pub const IDENT: &str = "position_is_behind_position";

impl Operation for PositionIsBehindPositionOp {
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
