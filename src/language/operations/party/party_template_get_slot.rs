use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PartyTemplateGetSlotOp;

const DOC : &str = "party_template_slot_eq                =  544   (party_template_slot_eq, <party_template_id>, <slot_no>, <value>),";

pub const OP_CODE: u32 = 524;

pub const IDENT: &str = "party_template_get_slot";

impl Operation for PartyTemplateGetSlotOp {
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
                make_param_doc("<party_template_id>", ""),
                make_param_doc("<slot_no>", ""),
            ],
        }
    }
}
