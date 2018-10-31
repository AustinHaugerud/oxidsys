use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PropInstanceAnimateToPositionOp;

const DOC : &str = "Moves prop instance to another position during the specified time frame (i.e. animates). Time is specified in 1/100th of second.";

pub const OP_CODE: u32 = 1860;

pub const IDENT: &str = "prop_instance_animate_to_position";

impl Operation for PropInstanceAnimateToPositionOp {
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
            param_docs: vec![make_param_doc("<scene_prop_id>", "")],
        }
    }
}
