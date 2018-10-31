use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ServerGetAddToGameServersListOp;

const DOC: &str = "";

pub const OP_CODE: u32 = 485;

pub const IDENT: &str = "server_get_add_to_game_servers_list";

impl Operation for ServerGetAddToGameServersListOp {
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
            num_required: 1,
            num_optional: 0,
            param_docs: vec![make_param_doc("<destination>", "")],
        }
    }
}
