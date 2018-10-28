use language::operations::Operation;

pub struct PlayerSavePickedUpItemsForNextSpawnOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 459;

pub const IDENT: &str = "player_save_picked_up_items_for_next_spawn";

impl Operation for PlayerSavePickedUpItemsForNextSpawnOp {
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
