use language::operations::Operation;

pub struct ServerSetAddToGameServersListOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 486;

pub const IDENT: &str = "server_set_add_to_game_servers_list";

impl Operation for ServerSetAddToGameServersListOp {
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
