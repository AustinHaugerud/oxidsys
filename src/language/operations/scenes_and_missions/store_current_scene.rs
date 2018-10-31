use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StoreCurrentSceneOp;

const DOC : &str = "Retrieves the identifier of the current scene. Note that the operation will return the scene id even after the mission is completed and the player is already on global map.";

pub const OP_CODE: u32 = 2211;

pub const IDENT: &str = "store_current_scene";

impl Operation for StoreCurrentSceneOp {
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
