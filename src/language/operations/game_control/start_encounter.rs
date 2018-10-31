use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StartEncounterOp;

const DOC : &str = "Forces the player party to initiate encounter with the specified party. Distance does not matter in this situation.";

pub const OP_CODE: u32 = 1300;

pub const IDENT: &str = "start_encounter";

impl Operation for StartEncounterOp {
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
