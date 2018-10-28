use language::operations::Operation;

pub struct MapGetRandomPositionAroundPositionOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1627;

pub const IDENT: &str = "map_get_random_position_around_position";

impl Operation for MapGetRandomPositionAroundPositionOp {
    fn op_code(&self) -> u32 {
        OP_CODE
    }

    fn documentation(&self) -> &'static str {
        DOC
    }

    fn identifier(&self) -> &'static str {
        IDENT
    }
}
