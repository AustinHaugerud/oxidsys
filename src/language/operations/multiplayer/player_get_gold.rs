use language::operations::Operation;

pub struct PlayerGetGoldOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 407;

pub const IDENT: &str = "player_get_gold";

impl Operation for PlayerGetGoldOp {
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
