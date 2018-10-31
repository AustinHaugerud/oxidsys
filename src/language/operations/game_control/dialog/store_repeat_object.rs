use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StoreRepeatObjectOp;

const DOC : &str = "Used in the dialogs code in combination with repeat_for_* dialog parameters, when creating dynamical player responses. Stores the value for the current iteration (i.e. a faction ID when repeat_for_factions is used, etc).";

pub const OP_CODE: u32 = 50;

pub const IDENT: &str = "store_repeat_object";

impl Operation for StoreRepeatObjectOp {
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
