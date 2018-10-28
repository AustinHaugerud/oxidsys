use language::operations::Operation;

pub struct MapGetWaterPositionAroundPositionOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1629;

pub const IDENT: &str = "map_get_water_position_around_position";

impl Operation for MapGetWaterPositionAroundPositionOp {
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
