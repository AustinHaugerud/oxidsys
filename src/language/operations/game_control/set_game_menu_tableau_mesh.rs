use language::operations::Operation;

pub struct SetGameMenuTableauMeshOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2032;

pub const IDENT: &str = "set_game_menu_tableau_mesh";

impl Operation for SetGameMenuTableauMeshOp {
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
