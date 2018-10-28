use language::operations::Operation;

pub struct GetDistanceBetweenPositionsInMetersOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 711;

pub const IDENT: &str = "get_distance_between_positions_in_meters";

impl Operation for GetDistanceBetweenPositionsInMetersOp {
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
