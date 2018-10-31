use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PartyTemplateSetSlotOp;

const DOC : &str = "party_template_get_slot               =  524   (party_template_get_slot, <destination>, <party_template_id>, <slot_no>),";

pub const OP_CODE: u32 = 504;

pub const IDENT: &str = "party_template_set_slot";

impl Operation for PartyTemplateSetSlotOp {
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
                make_param_doc("<party_template_id>", ""),
                make_param_doc("<slot_no>", ""),
                make_param_doc("<value>", ""),
            ],
        }
    }
}
