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

    pub fn request_documentation(&self, op_identifier: &str) -> &str {
        if let Some(op) = self.operations.get(op_identifier) {
            op.documentation()
        } else {
            INVALID_OP_RESPONSE
        }
    }
}
