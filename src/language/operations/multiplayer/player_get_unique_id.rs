use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PlayerGetUniqueIdOp;

const DOC : &str = "Server operation. Retrieves player's unique identifier which is determined by player's game license code. This number is supposed to be unique for each license, allowing reliable player identification across servers.";

pub const OP_CODE: u32 = 441;

pub const IDENT: &str = "player_get_unique_id";

impl Operation for PlayerGetUniqueIdOp {
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
            num_required: 2,
            num_optional: 0,
            param_docs: vec![
                make_param_doc("<destination>", ""),
                make_param_doc("<player_id>", ""),
            ],
        }
    }
}
