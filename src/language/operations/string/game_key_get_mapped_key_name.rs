use language::operations::Operation;

pub struct GameKeyGetMappedKeyNameOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 65;

pub const IDENT: &str = "game_key_get_mapped_key_name";

impl Operation for GameKeyGetMappedKeyNameOp {
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
