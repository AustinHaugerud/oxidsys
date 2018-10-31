use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PropInstanceGetScaleOp;

const DOC: &str = "Retrieves the current scaling factors of the prop instance.";

pub const OP_CODE: u32 = 1852;

pub const IDENT: &str = "prop_instance_get_scale";

impl Operation for PropInstanceGetScaleOp {
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
                make_param_doc("<position>", ""),
                make_param_doc("<scene_prop_id>", ""),
            ],
        }
    }
}
