use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct SpawnAroundPartyOp;

const DOC: &str = "Creates a new party from a party template and puts it's <party_id> into reg0.";

pub const OP_CODE: u32 = 1100;

pub const IDENT: &str = "spawn_around_party";

impl Operation for SpawnAroundPartyOp {
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
                make_param_doc("<party_template_id>", ""),
            ],
        }
    }
}
