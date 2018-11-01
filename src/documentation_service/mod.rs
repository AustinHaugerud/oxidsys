use std::collections::HashMap;

use language::operations::Operation;

const INVALID_OP_RESPONSE: &str = r#"
Unrecognized operand. Please enter an identifier that exists.
Note: deprecated operands are not recognized by oxidsys.
"#;

pub struct DocumentationService<'a> {
    operations: &'a HashMap<&'static str, Box<Operation>>,
}

impl<'a> DocumentationService<'a> {
    pub fn new(map: &'a HashMap<&'static str, Box<Operation>>) -> DocumentationService<'a> {
        DocumentationService { operations: map }
    }

    pub fn request_documentation(&self, op_identifier: &str) -> String {
        if let Some(op) = self.operations.get(op_identifier) {
            compile_docs(op)
        } else {
            INVALID_OP_RESPONSE.to_string()
        }
    }
}

fn compile_docs(operand: &Box<Operation>) -> String {
    let mut result = operand.documentation().to_string();
    for param in operand.param_info().get_param_docs() {
        result += &format!("{}: {}\n", param.0, param.1);
    }

    result
}
