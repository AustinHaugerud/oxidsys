use language::operations::Operation;

pub struct TryForPlayersOp;

const DOC: &str = r#"
Iterate all players in multiplayer. Set optional parameter to 1 to skip server player entry.
Format: try_for_players <iterator> [skip_server=0];
"#;

const OP_CODE: u16 = 17;

const IDENT: &str = "try_for_players";

impl Operation for TryForPlayersOp {
    fn op_code(&self) -> u16 {
        OP_CODE
    }

    fn documentation(&self) -> &str {
        DOC
    }

    fn identifier(&self) -> &str {
        IDENT
    }
}
