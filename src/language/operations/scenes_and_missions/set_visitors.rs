use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct SetVisitorsOp;

const DOC: &str = "Save as (set_visitors), but spawns an entire group of some troop type.";

pub const OP_CODE: u32 = 1264;

pub const IDENT: &str = "set_visitors";

impl Operation for SetVisitorsOp {
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
                make_param_doc("<entry_no>", ""),
                make_param_doc("<troop_id>", ""),
                make_param_doc("<number_of_troops>", ""),
            ],
        }
    }
}
