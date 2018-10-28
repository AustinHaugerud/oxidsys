use language::operations::Operation;

pub struct ServerGetAddToGameServersListOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 485;

pub const IDENT: &str = "server_get_add_to_game_servers_list";

impl Operation for ServerGetAddToGameServersListOp {
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
