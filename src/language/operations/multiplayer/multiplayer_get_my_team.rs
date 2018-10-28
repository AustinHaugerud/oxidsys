use language::operations::Operation;

pub struct MultiplayerGetMyTeamOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 411;

pub const IDENT: &str = "multiplayer_get_my_team";

impl Operation for MultiplayerGetMyTeamOp {
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
