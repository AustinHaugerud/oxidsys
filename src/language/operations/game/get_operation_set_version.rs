use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct GetOperationSetVersionOp;

const DOC : &str = "Version 1.165+. 4research. Apparently returns the current version of Module System operations set, allowing transparent support for multiple Warband engine versions.";

pub const OP_CODE: u32 = 55;

pub const IDENT: &str = "get_operation_set_version";

impl Operation for GetOperationSetVersionOp {
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
