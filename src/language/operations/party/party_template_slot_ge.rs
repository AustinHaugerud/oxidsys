use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PartyTemplateSlotGeOp;

const DOC: &str = "";

pub const OP_CODE: u32 = 564;

pub const IDENT: &str = "party_template_slot_ge";

impl Operation for PartyTemplateSlotGeOp {
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
