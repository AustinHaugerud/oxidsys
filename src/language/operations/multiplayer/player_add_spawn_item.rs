use language::operations::Operation;

pub struct PlayerAddSpawnItemOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 410;

pub const IDENT: &str = "player_add_spawn_item";

impl Operation for PlayerAddSpawnItemOp {
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
