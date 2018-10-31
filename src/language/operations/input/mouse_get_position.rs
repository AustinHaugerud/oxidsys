use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct MouseGetPositionOp;

const DOC: &str = "Stores mouse x and y coordinates in the specified position.";

pub const OP_CODE: u32 = 75;

pub const IDENT: &str = "mouse_get_position";

impl Operation for MouseGetPositionOp {
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
