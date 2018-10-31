use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ShowObjectDetailsOverlayOp;

const DOC : &str = "Turns various popup tooltips on (value = 1) and off (value = 0). This includes agent names and dropped item names during missions, item stats in inventory on mouse over, etc.";

pub const OP_CODE: u32 = 960;

pub const IDENT: &str = "show_object_details_overlay";

impl Operation for ShowObjectDetailsOverlayOp {
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
            param_docs: vec![make_param_doc("<value>", "")],
        }
    }
}
