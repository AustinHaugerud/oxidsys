use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct SetShaderParamIntOp;

const DOC : &str = "Version 1.153+. UNTESTED. Allows direct manupulation of shader parameters. Operation scope is unknown, possibly global. Parameter is an int value.";

pub const OP_CODE: u32 = 2400;

pub const IDENT: &str = "set_shader_param_int";

impl Operation for SetShaderParamIntOp {
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
            num_required: 4,
            num_optional: 0,
            param_docs: vec![
                make_param_doc("<parameter_name>", ""),
                make_param_doc("<value>", ""),
                make_param_doc("<parameter_name>", ""),
                make_param_doc("<value>", ""),
            ],
        }
    }
}
