use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct DisablePartyOp;

const DOC : &str = "Party disappears from the map. Note that (try_for_parties) will still iterate over disabled parties, so you need to make additional checks with (party_is_active).";

pub const OP_CODE: u32 = 1230;

pub const IDENT: &str = "disable_party";

impl Operation for DisablePartyOp {
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
