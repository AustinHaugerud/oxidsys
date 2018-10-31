use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PartySetFlagsOp;

const DOC: &str =
    "Sets (1) or clears (0) party flags in runtime. See header_parties.py for flags reference.";

pub const OP_CODE: u32 = 1603;

pub const IDENT: &str = "party_set_flags";

impl Operation for PartySetFlagsOp {
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
                make_param_doc("<party_id>", ""),
                make_param_doc("<flag>", ""),
                make_param_doc("<clear_or_set>", ""),
            ],
        }
    }
}
