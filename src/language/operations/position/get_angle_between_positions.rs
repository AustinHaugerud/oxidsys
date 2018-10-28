use language::operations::Operation;

pub struct GetAngleBetweenPositionsOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 705;

pub const IDENT: &str = "get_angle_between_positions";

impl Operation for GetAngleBetweenPositionsOp {
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
