use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PropInstanceGetCurrentDeformFrameOp;

const DOC : &str = "Version 1.161+. Returns current frame of a vertex-animated scene prop, rounded to nearest integer value.";

pub const OP_CODE: u32 = 2616;

pub const IDENT: &str = "prop_instance_get_current_deform_frame";

impl Operation for PropInstanceGetCurrentDeformFrameOp {
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
