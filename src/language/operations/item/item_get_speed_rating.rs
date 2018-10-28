use language::operations::Operation;

pub struct ItemGetSpeedRatingOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2708;

pub const IDENT: &str = "item_get_speed_rating";

impl Operation for ItemGetSpeedRatingOp {
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
