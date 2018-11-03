use language::operations::{Operation, ParamInfo};

pub struct SceneAllowsMountedUnitsOp;

const DOC : &str = "Not documented. Used in multiplayer, but it's not clear where horses could be disallowed in the first place. 4research.";

pub const OP_CODE: u32 = 1834;

pub const IDENT: &str = "scene_allows_mounted_units";

impl Operation for SceneAllowsMountedUnitsOp {
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
