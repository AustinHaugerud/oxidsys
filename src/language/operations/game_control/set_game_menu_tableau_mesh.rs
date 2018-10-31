use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct SetGameMenuTableauMeshOp;

const DOC : &str = "Adds a tableau to the current game menu screen. Position (X,Y) coordinates define mesh position, Z coordinate defines scaling. Parameter <value> will be passed as tableau_material script parameter.";

pub const OP_CODE: u32 = 2032;

pub const IDENT: &str = "set_game_menu_tableau_mesh";

impl Operation for SetGameMenuTableauMeshOp {
    fn op_code(&self) -> u32 {
        OP_CODE
    }

    fn documentation(&self) -> &'static str {
        DOC
    }

    fn identifier(&self) -> &'static str {
        IDENT
    }

    fn param_info(&self) -> ParamInfo {
        ParamInfo {
            num_required: 3,
            num_optional: 0,
            param_docs: vec![
                make_param_doc("<tableau_material_id>", ""),
                make_param_doc("<value>", ""),
                make_param_doc("<position_register_no>", ""),
            ],
        }
    }
}
