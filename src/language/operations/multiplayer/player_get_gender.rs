use language::operations::Operation;

pub struct PlayerGetGenderOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 442;

pub const IDENT: &str = "player_get_gender";

impl Operation for PlayerGetGenderOp {
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
