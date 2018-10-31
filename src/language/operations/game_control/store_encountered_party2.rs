use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StoreEncounteredParty2Op;

const DOC : &str = "Stores the identifier of the second encountered party (when first party is in battle, this one will return it's battle opponent).";

pub const OP_CODE: u32 = 2203;

pub const IDENT: &str = "store_encountered_party2";

impl Operation for StoreEncounteredParty2Op {
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
