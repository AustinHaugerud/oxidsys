use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PartyIgnorePlayerOp;

const DOC: &str = "Makes AI party ignore player for the specified time.";

pub const OP_CODE: u32 = 1644;

pub const IDENT: &str = "party_ignore_player";

impl Operation for PartyIgnorePlayerOp {
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
                make_param_doc("<party_id>", ""),
                make_param_doc("<duration_in_hours>", ""),
            ],
        }
    }
}
