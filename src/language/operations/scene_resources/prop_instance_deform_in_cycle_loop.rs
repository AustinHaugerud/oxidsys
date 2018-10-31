use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PropInstanceDeformInCycleLoopOp;

const DOC : &str = "Version 1.161+. Performs looping animation of vertex-animated scene prop within the specified vertex frame ranges and within specified time (in milliseconds). If you open the mesh in OpenBrf, right one of \"Time of frame\" boxes contains the relevant values for frame parameters.";

pub const OP_CODE: u32 = 2612;

pub const IDENT: &str = "prop_instance_deform_in_cycle_loop";

impl Operation for PropInstanceDeformInCycleLoopOp {
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
            num_required: 3,
            num_optional: 0,
            param_docs: vec![
                make_param_doc("<prop_instance_no>", ""),
                make_param_doc("<start_frame>", ""),
                make_param_doc("<end_frame>", ""),
            ],
        }
    }
}
