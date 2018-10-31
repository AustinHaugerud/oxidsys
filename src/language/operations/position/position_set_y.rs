use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PositionSetYOp;

const DOC: &str = "Set position Y coordinate.";

pub const OP_CODE: u32 = 730;

pub const IDENT: &str = "position_set_y";

impl Operation for PositionSetYOp {
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
                make_param_doc("<value_fixed_point>", ""),
            ],
        }
    }
}
