use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StoreAtan2Op;

const DOC : &str = "Returns the angle between the x axis and a point with coordinates (X,Y) in degrees. Note the angle is calculated counter-clockwise, i.e. (1,1) will return 45, not -45.";

pub const OP_CODE: u32 = 2143;

pub const IDENT: &str = "store_atan2";

impl Operation for StoreAtan2Op {
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
                make_param_doc("<destination_fixed_point>", ""),
                make_param_doc("<y_fixed_point>", ""),
                make_param_doc("<x_fixed_point>", ""),
            ],
        }
    }
}
