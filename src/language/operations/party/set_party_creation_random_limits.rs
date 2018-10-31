use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct SetPartyCreationRandomLimitsOp;

const DOC : &str = "Affects party sizes spawned from templates. May be used to spawn larger parties when player is high level. Values should be in 0..100 range.";

pub const OP_CODE: u32 = 1080;

pub const IDENT: &str = "set_party_creation_random_limits";

impl Operation for SetPartyCreationRandomLimitsOp {
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
                make_param_doc("<min_value>", ""),
                make_param_doc("<max_value>", ""),
            ],
        }
    }
}
