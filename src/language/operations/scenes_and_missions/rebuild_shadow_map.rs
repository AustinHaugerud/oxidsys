use language::operations::{Operation, ParamInfo};

pub struct RebuildShadowMapOp;

const DOC : &str = "Version 1.153+. UNTESTED. Effects unknown. Rebuilds shadow map for the current scene. Apparently useful after heavy manipulation with scene props.";

pub const OP_CODE: u32 = 2393;

pub const IDENT: &str = "rebuild_shadow_map";

impl Operation for RebuildShadowMapOp {
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
