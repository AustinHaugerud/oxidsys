use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PropInstanceIsValidOp;

const DOC: &str =
    "Checks that the reference to a scene prop instance is valid (i.e. it was not removed).";

pub const OP_CODE: u32 = 1838;

pub const IDENT: &str = "prop_instance_is_valid";

impl Operation for PropInstanceIsValidOp {
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
            param_docs: vec![make_param_doc("<scene_prop_instance_id>", "")],
        }
    }
}
