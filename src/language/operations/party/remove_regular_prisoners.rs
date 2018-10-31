use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct RemoveRegularPrisonersOp;

const DOC: &str = "Removes all non-hero prisoners from the party.";

pub const OP_CODE: u32 = 1211;

pub const IDENT: &str = "remove_regular_prisoners";

impl Operation for RemoveRegularPrisonersOp {
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
