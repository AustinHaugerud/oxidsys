use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PropInstanceGetCurrentDeformProgressOp;

const DOC : &str = "Version 1.161+. Returns a percentage value between 0 and 100 if animation is still in progress. Returns 100 otherwise.";

pub const OP_CODE: u32 = 2615;

pub const IDENT: &str = "prop_instance_get_current_deform_progress";

impl Operation for PropInstanceGetCurrentDeformProgressOp {
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
                make_param_doc("<prop_instance_no>", ""),
            ],
        }
    }
}
