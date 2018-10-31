use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ShowTroopDetailsOp;

const DOC : &str = "Version 1.153+. Supposedly displays a popup with troop information at specified place. 4research.";

pub const OP_CODE: u32 = 2388;

pub const IDENT: &str = "show_troop_details";

impl Operation for ShowTroopDetailsOp {
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
                make_param_doc("<troop_id>", ""),
                make_param_doc("<position>", ""),
                make_param_doc("<troop_price>", ""),
            ],
        }
    }
}
