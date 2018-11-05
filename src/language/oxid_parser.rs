use std::collections::HashMap;

#[derive(Parser)]
#[grammar = "language/oxid.pest"]
struct OxidParser;

fn filter(input: &str) -> String {
    input.replace("\n", "").replace("\r", "").replace("\t", "")
}

#[derive(Debug)]
pub struct OpCall {
    op_id : String,
    args : Vec<String>,
}

impl OpCall {
    pub fn get_op_id(&self) -> &str {
        &self.op_id
    }

    pub fn get_args(&self) -> &Vec<String> {
        &self.args
    }
}

#[derive(Copy, Clone, Debug)]
pub enum RegisterKind {
    Numeric,
    String,
    Position,
}

lazy_static! {
    static ref REG_TYPE_MAP : HashMap<&'static str, RegisterKind> = {
        let mut m = HashMap::new();
        m.insert("register", RegisterKind::Numeric);
        m.insert("sregister", RegisterKind::String);
        m.insert("pregister", RegisterKind::Position);
        m
    };
}

#[derive(Debug)]
pub struct RegisterDecl {
    kind : RegisterKind,
    reg_id : String,
}

impl RegisterDecl {
    pub fn get_kind(&self) -> RegisterKind {
        self.kind
    }

    pub fn get_reg_id(&self) -> &str {
        &self.reg_id
    }
}

#[derive(Debug)]
pub struct ParamDecl {
    pos : u32,
    id : String,
}

impl ParamDecl {
    pub fn get_pos(&self) -> u32 {
        self.pos
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }
}

lazy_static! {
    static ref VAR_SCOPE_MAP : HashMap<&'static str, VarScope> = {
        let mut m = HashMap::new();
        m.insert("local", VarScope::Local);
        m.insert("global", VarScope::Global);
        m
    };
}

#[derive(Copy, Clone, Debug)]
pub enum VarScope {
    Local,
    Global,
}

#[derive(Debug)]
pub struct VarDecl {
    id : String,
    scope : VarScope,
    init_instructions : Vec<OpCall>
}

impl VarDecl {
    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_scope(&self) -> VarScope {
        self.scope
    }

    pub fn get_init_instructions(&self) -> &Vec<OpCall> {
        &self.init_instructions
    }
}

#[derive(Debug)]
pub struct VarChange {
    id : String,
    change_instructions : Vec<OpCall>
}

#[derive(Debug)]
pub enum CollectedStatement {
    RegisterDeclaration(RegisterDecl),
    ParamDeclaration(ParamDecl),
    VariableDeclaration(VarDecl),
    VariableChange(VarChange),
    OperandCall(OpCall)
}

pub fn parse_script(content : &str) -> Result<Vec<CollectedStatement>, String> {
    use pest::Parser;

    let filtered_content = filter(content);
    let mut result = OxidParser::parse(Rule::file, &filtered_content).map_err(|e| format!("{}", e))?;

    let mut statements : Vec<CollectedStatement> = vec![];

    let mut param_index_counter = 0u32;

    let statement_block = result.next();
    if statement_block.is_none() {
        return Ok(statements);
    }

    for line in statement_block.expect("Expected statements").into_inner() {
        match line.as_rule() {
            Rule::statement => {
                let mut statement = line.into_inner().next().expect("Expected statement");
                match statement.as_rule() {
                    Rule::any_register_declaration => {
                        let mut register = statement.into_inner();
                        let rtype = register.next().expect("Expected rtype").as_str();
                        let id = register.next().expect("Expected rid").as_str().to_owned();
                        let decl = RegisterDecl {
                            kind : REG_TYPE_MAP[rtype],
                            reg_id : id
                        };
                        statements.push(CollectedStatement::RegisterDeclaration(decl));
                    },
                    Rule::param_declaration => {
                        let mut param = statement.into_inner();
                        let id = param.next().expect("Expected pid").as_str();
                        let decl = ParamDecl {
                            pos : param_index_counter,
                            id : id.to_owned(),
                        };
                        param_index_counter += 1;
                        statements.push(CollectedStatement::ParamDeclaration(decl));
                    },
                    Rule::variable_declaration => {
                        let mut var_decl = statement.into_inner();
                        let scope = var_decl.next().expect("Expected variable scope").as_str();
                        let id =  var_decl.next().expect("Expected vid").as_str();
                        println!("Warning: Expressions not implemented.");
                        let decl = VarDecl {
                            id : id.to_owned(),
                            scope : VAR_SCOPE_MAP[scope],
                            init_instructions: vec![]
                        };
                        statements.push(CollectedStatement::VariableDeclaration(decl));

                    },
                    Rule::variable_change => {
                        println!("Warning: Expressions not implemented.");
                    },
                    Rule::operand_call => {
                        let mut opcall = statement.into_inner();
                        let id = opcall.next().expect("Expected oid").as_str();
                        let mut args = vec![];
                        while let Some(arg) = opcall.next() {
                            args.push(arg.as_str().to_owned());
                        }
                        let call = OpCall {
                            op_id: id.to_owned(),
                            args: args
                        };
                        statements.push(CollectedStatement::OperandCall(call));
                    },
                    _ => {},
                }
            },
            _ => {}
        }
    }

    for s in statements.iter() {
        println!("{:?}", s);
    }

    Ok(statements)
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn test1() {
        let content = r#"
            param p1;
            param p2;
            param p3;
            register reg20;

            local b;
            assign b 10;
            global c;

            register reg10;
            register reg11;
            register reg40;

            pregister p10;
            pregister p11;

            sregister s3;
            sregister s12;
        "#;

        parse_script(content).unwrap();
    }

}
