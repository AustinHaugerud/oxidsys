use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct InitPositionOp;

const DOC: &str = "Sets position coordinates to [0,0,0], without any rotation and default scale.";

pub const OP_CODE: u32 = 701;

pub const IDENT: &str = "init_position";

impl Operation for InitPositionOp {
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
            param_docs: vec![make_param_doc("<position>", "")],
        }
    }
}
