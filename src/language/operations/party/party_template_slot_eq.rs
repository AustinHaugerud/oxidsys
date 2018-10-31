use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PartyTemplateSlotEqOp;

const DOC : &str = "party_template_slot_ge                =  564   (party_template_slot_ge, <party_template_id>, <slot_no>, <value>),";

pub const OP_CODE: u32 = 544;

pub const IDENT: &str = "party_template_slot_eq";

impl Operation for PartyTemplateSlotEqOp {
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
