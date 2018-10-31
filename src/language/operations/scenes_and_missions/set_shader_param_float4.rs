use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct SetShaderParamFloat4Op;

const DOC : &str = "Version 1.153+. Allows direct manupulation of shader parameters. Operation scope is unknown, possibly global. Parameter is a set of 4 float values.";

pub const OP_CODE: u32 = 2402;

pub const IDENT: &str = "set_shader_param_float4";

impl Operation for SetShaderParamFloat4Op {
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
            num_required: 5,
            num_optional: 0,
            param_docs: vec![
                make_param_doc("<parameter_name>", ""),
                make_param_doc("<valuex>", ""),
                make_param_doc("<valuey>", ""),
                make_param_doc("<valuez>", ""),
                make_param_doc("<valuew>", ""),
            ],
        }
    }
}
