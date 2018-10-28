use language::operations::Operation;

pub struct MapGetLandPositionAroundPositionOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1628;

pub const IDENT: &str = "map_get_land_position_around_position";

impl Operation for MapGetLandPositionAroundPositionOp {
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
