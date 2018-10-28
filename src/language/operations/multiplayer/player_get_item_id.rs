use language::operations::Operation;

pub struct PlayerGetItemIdOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 422;

pub const IDENT: &str = "player_get_item_id";

impl Operation for PlayerGetItemIdOp {
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
