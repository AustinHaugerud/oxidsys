use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PartyCountPrisonersOfTypeOp;

const DOC: &str = "Returns total number of prisoners of specific type.";

pub const OP_CODE: u32 = 1632;

pub const IDENT: &str = "party_count_prisoners_of_type";

impl Operation for PartyCountPrisonersOfTypeOp {
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
                make_param_doc("<party_id>", ""),
                make_param_doc("<troop_id>", ""),
            ],
        }
    }
}
