use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StoreEncounteredPartyOp;

const DOC: &str = "Stores identifier of the encountered party.";

pub const OP_CODE: u32 = 2202;

pub const IDENT: &str = "store_encountered_party";

impl Operation for StoreEncounteredPartyOp {
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
            param_docs: vec![make_param_doc("<destination>", "")],
        }
    }
}
