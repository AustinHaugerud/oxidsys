use language::operations::Operation;

pub struct PlayerGetUniqueIdOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 441;

pub const IDENT: &str = "player_get_unique_id";

impl Operation for PlayerGetUniqueIdOp {
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
