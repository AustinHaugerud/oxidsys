use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StrEncodeUrlOp;

const DOC : &str = "This operation will \"sanitize\" a string to be used as part of network URL, replacing any non-standard characters with their '%'-codes.";

pub const OP_CODE: u32 = 2355;

pub const IDENT: &str = "str_encode_url";

impl Operation for StrEncodeUrlOp {
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
            param_docs: vec![make_param_doc("<string_register>", "")],
        }
    }
}
