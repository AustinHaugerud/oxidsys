use language::operations::Operation;

pub struct GetMaxPlayersOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 400;

pub const IDENT: &str = "get_max_players";

impl Operation for GetMaxPlayersOp {
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
