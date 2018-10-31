use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct CreateNumberBoxOverlayOp;

const DOC : &str = "Creates a number box overlay (a small field for numeric value and small increase/decrease buttons to the right) with specified min and max values. Returns number box overlay_id.";

pub const OP_CODE: u32 = 942;

pub const IDENT: &str = "create_number_box_overlay";

impl Operation for CreateNumberBoxOverlayOp {
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
                make_param_doc("<destination>", ""),
                make_param_doc("<min_value>", ""),
                make_param_doc("<max_value>", ""),
            ],
        }
    }
}
