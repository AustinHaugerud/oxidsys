use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PropInstanceGetStartingPositionOp;

const DOC : &str = "Retrieves the prop instance starting position on the scene (i.e. the place where it was positioned when initialized).";

pub const OP_CODE: u32 = 1851;

pub const IDENT: &str = "prop_instance_get_starting_position";

impl Operation for PropInstanceGetStartingPositionOp {
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
