use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PropInstanceGetVariationIdOp;

const DOC: &str = "Retrieves the first variation ID number for the specified scene prop instance.";

pub const OP_CODE: u32 = 1840;

pub const IDENT: &str = "prop_instance_get_variation_id";

impl Operation for PropInstanceGetVariationIdOp {
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
                make_param_doc("<scene_prop_id>", ""),
            ],
        }
    }
}
