use language::operations::Operation;

pub struct PlayerGetValueOfOriginalItemsOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 460;

pub const IDENT: &str = "player_get_value_of_original_items";

impl Operation for PlayerGetValueOfOriginalItemsOp {
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
