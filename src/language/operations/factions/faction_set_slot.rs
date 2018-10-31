use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct FactionSetSlotOp;

const DOC : &str = "faction_get_slot                =  522   (faction_get_slot, <destination>, <faction_id>, <slot_no>),";

pub const OP_CODE: u32 = 502;

pub const IDENT: &str = "faction_set_slot";

impl Operation for FactionSetSlotOp {
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
                make_param_doc("<faction_id>", ""),
                make_param_doc("<slot_no>", ""),
                make_param_doc("<value>", ""),
            ],
        }
    }
}
