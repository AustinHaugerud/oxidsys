use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PartyGetMoraleOp;

const DOC : &str = "Returns a value in the range of 0..100. Party morale does not affect party behavior on the map, but will be taken in account if the party is engaged in battle (except auto-calc).";

pub const OP_CODE: u32 = 1671;

pub const IDENT: &str = "party_get_morale";

impl Operation for PartyGetMoraleOp {
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
            num_optional: 0,
            param_docs: vec![
                make_param_doc("<destination>", ""),
                make_param_doc("<party_id>", ""),
            ],
        }
    }
}
