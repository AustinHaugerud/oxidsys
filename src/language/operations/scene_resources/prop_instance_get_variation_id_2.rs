use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PropInstanceGetVariationId2Op;

const DOC: &str = "Retrieves the second variation ID number for the specified scene prop instance.";

pub const OP_CODE: u32 = 1841;

pub const IDENT: &str = "prop_instance_get_variation_id_2";

impl Operation for PropInstanceGetVariationId2Op {
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
