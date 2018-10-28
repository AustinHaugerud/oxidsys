use language::operations::Operation;

pub struct ItemGetHorseManeuverOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2715;

pub const IDENT: &str = "item_get_horse_maneuver";

impl Operation for ItemGetHorseManeuverOp {
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
