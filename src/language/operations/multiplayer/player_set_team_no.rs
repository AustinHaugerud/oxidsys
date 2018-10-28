use language::operations::Operation;

pub struct PlayerSetTeamNoOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 403;

pub const IDENT: &str = "player_set_team_no";

impl Operation for PlayerSetTeamNoOp {
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
