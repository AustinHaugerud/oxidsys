use language::operations::{Operation, ParamInfo};

pub struct MapFreeOp;

const DOC: &str =
    "Checks that the player is currently on the global map and no game screens are open.";

pub const OP_CODE: u32 = 37;

pub const IDENT: &str = "map_free";

impl Operation for MapFreeOp {
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
            num_required: 0,
            num_optional: 0,
            param_docs: vec![],
        }
    }
}
