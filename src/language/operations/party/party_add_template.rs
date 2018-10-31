use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PartyAddTemplateOp;

const DOC : &str = "Reinforces the party using the specified party template. Optional flag switches troop/prisoner status for reinforcements.";

pub const OP_CODE: u32 = 1675;

pub const IDENT: &str = "party_add_template";

impl Operation for PartyAddTemplateOp {
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
            num_required: 2,
            num_optional: 1,
            param_docs: vec![
                make_param_doc("<party_id>", ""),
                make_param_doc("<party_template_id>", ""),
                make_param_doc("[reverse_prisoner_status]", ""),
            ],
        }
    }
}
