use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct RemovePartyOp;

const DOC : &str = "Destroys a party completely. Should ONLY be used with dynamically spawned parties, as removing parties pre-defined in module_parties.py file will corrupt the savegame.";

pub const OP_CODE: u32 = 1232;

pub const IDENT: &str = "remove_party";

impl Operation for RemovePartyOp {
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
