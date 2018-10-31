use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PartySetSlotOp;

const DOC : &str = "party_get_slot                        =  521   (party_get_slot, <destination>, <party_id>, <slot_no>),";

pub const OP_CODE: u32 = 501;

pub const IDENT: &str = "party_set_slot";

impl Operation for PartySetSlotOp {
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
                make_param_doc("<party_id>", ""),
                make_param_doc("<slot_no>", ""),
                make_param_doc("<value>", ""),
            ],
        }
    }
}
