use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct FactionGetSlotOp;

const DOC: &str =
    "faction_slot_eq                 =  542   (faction_slot_eq, <faction_id>, <slot_no>, <value>),";

pub const OP_CODE: u32 = 522;

pub const IDENT: &str = "faction_get_slot";

impl Operation for FactionGetSlotOp {
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
                make_param_doc("<faction_id>", ""),
                make_param_doc("<slot_no>", ""),
            ],
        }
    }
}
