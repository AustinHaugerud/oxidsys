use language::operations::Operation;

pub struct PlayerGetTeamNoOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 402;

pub const IDENT: &str = "player_get_team_no";

impl Operation for PlayerGetTeamNoOp {
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
