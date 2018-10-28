use language::operations::Operation;

pub struct MultiplayerFindSpawnPointOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 425;

pub const IDENT: &str = "multiplayer_find_spawn_point";

impl Operation for MultiplayerFindSpawnPointOp {
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
