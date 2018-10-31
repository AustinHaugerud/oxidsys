use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct SetPositionDeltaOp;

const DOC : &str = "Can only be called inside item or scene prop triggers. Sets (X,Y,Z) offsets from the item/prop current position for subsequent calls to (add_point_light) etc. Offsets are apparently in centimeters.";

pub const OP_CODE: u32 = 1955;

pub const IDENT: &str = "set_position_delta";

impl Operation for SetPositionDeltaOp {
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
                make_param_doc("<value>", ""),
                make_param_doc("<value>", ""),
                make_param_doc("<value>", ""),
            ],
        }
    }
}
