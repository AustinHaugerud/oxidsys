use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct HealPartyOp;

const DOC: &str = "Heals all wounded party members.";

pub const OP_CODE: u32 = 1225;

pub const IDENT: &str = "heal_party";

impl Operation for HealPartyOp {
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
            num_required: 1,
            num_optional: 0,
            param_docs: vec![make_param_doc("<party_id>", "")],
        }
    }
}
