use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PartySetNameOp;

const DOC: &str = "Sets party name (will be displayed as label and/or in the party details popup).";

pub const OP_CODE: u32 = 1669;

pub const IDENT: &str = "party_set_name";

impl Operation for PartySetNameOp {
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
                make_param_doc("<string>", ""),
            ],
        }
    }
}
