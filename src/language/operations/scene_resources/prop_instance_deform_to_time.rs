use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PropInstanceDeformToTimeOp;

const DOC : &str = "Version 1.161+. Deforms a vertex-animated scene prop to specified vertex time. If you open the mesh in OpenBrf, right one of \"Time of frame\" boxes contains the relevant value.";

pub const OP_CODE: u32 = 2610;

pub const IDENT: &str = "prop_instance_deform_to_time";

impl Operation for PropInstanceDeformToTimeOp {
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
                make_param_doc("<prop_instance_no>", ""),
                make_param_doc("<value>", ""),
            ],
        }
    }
}
