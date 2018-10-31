use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ClearOmittedKeysOp;

const DOC : &str = "Commonly called when exiting from a presentation which made any calls to (omit_key_once). However the effects of those calls disappear by the next frame, so apparently usage of this operation is not necessary. It is still recommended to be on the safe side though.";

pub const OP_CODE: u32 = 78;

pub const IDENT: &str = "clear_omitted_keys";

impl Operation for ClearOmittedKeysOp {
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
            num_required: 0,
            num_optional: 0,
            param_docs: vec![],
        }
    }
}
