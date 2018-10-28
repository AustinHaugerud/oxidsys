use language::operations::Operation;

pub struct GetSqDistanceBetweenPositionHeightsOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 715;

pub const IDENT: &str = "get_sq_distance_between_position_heights";

impl Operation for GetSqDistanceBetweenPositionHeightsOp {
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
