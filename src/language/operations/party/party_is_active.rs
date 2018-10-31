use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PartyIsActiveOp;

const DOC: &str = "Checks that <party_id> is valid and not disabled.";

pub const OP_CODE: u32 = 132;

pub const IDENT: &str = "party_is_active";

impl Operation for PartyIsActiveOp {
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
