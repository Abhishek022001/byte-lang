use std::{collections::HashMap, panic};

use crate::datatypes::{ast_statements::{BuiltInFunctionsAst, CgBranchLinked, CgBuiltInFunctions, CgExpression, CgStatement, CgStatementType, CgVariableInitialization, Expression, Literal, Statement, Statements, VariableType}, program_data::ProgramData, stack_frame::{StackFrame, StackVariable}, token::{BuiltInFunctions, Identifiers}};

pub struct SemanticAnaytis<'a> {
    program_data : &'a mut ProgramData
}

impl<'a> SemanticAnaytis<'a> {
    pub fn new(program_data : &'a mut ProgramData) -> Self {
        Self {
            program_data
        }
    }

    pub fn process_statement(&mut self, statement : &'_ Statement, stack_frame : usize) -> Result<Option<CgStatement>, String> {
        match statement.statement_type.clone() {
            Statements::VariableDeclaration(var_init) => {
                if let Some(init_value) = var_init.value {
                    let init_valid = match (var_init.variable_type, init_value.clone()) {
                        (VariableType::I8  | VariableType::I16 | VariableType::I32,
                        Expression::Literal(Literal::Number(_))) => true,
                        _ => false
                    };

                    if !init_valid {
                        return Err(String::from("Invalid var Declaration"));
                    }

                    let cg_val = self.expression_to_cg(stack_frame, init_value);

                    self.add_cg_statement_to_stack_frame(stack_frame, CgStatement{statement_type: CgStatementType::VariableInitialization(CgVariableInitialization{init_value: cg_val, var_name: var_init.name, stack_frame: stack_frame})})
                };

                /*let var_stack_frame = self.get_stack_frame_by_index(var_init.stack_frame);
                let var_in_stack_frame = var_stack_frame.variables.get(&var_init.var_name).unwrap();

                let init_valid = match (var_in_stack_frame.variable_type.clone(), var_init) {
                    (VariableType::I8  | VariableType::I16 | VariableType::I32,
                    Expression::Literal(Literal::Number(_))) => true,
                    _ => false
                };

                if !init_valid {
                    return Err(String::from("Invalid var Declaration"));
                    //return Err(String::from(format!("Invalid Variable Declaration at line {} and col {}: {:?}", statement.line, statement.col, self.untokenized_input.get(statement.start_pos..statement.end_pos).unwrap()))); 
                }*/

                return Ok(None);
            },
            Statements::Expression(Expression::BuiltInFunction(func)) => {
                match func {
                    BuiltInFunctionsAst::BranchLinked(branch_linked) => {
                        if self.program_data.functions.get(&branch_linked.function_name).is_none() {
                            panic!("Branching to unknown function: {:?}", branch_linked.function_name);
                        }

                        let bl_function = self.program_data.functions.get(&branch_linked.function_name).unwrap().clone();

                        if branch_linked.args.len() != bl_function.args.len() {
                            panic!("")
                        }

                        let mut cg_args : Vec<CgExpression> = Vec::new();

                        let mut i = 0;
                        while i < branch_linked.args.len() {
                            let cg_expression = self.expression_to_cg(stack_frame, branch_linked.args.get(i).unwrap().clone());

                            let valid : bool = match (cg_expression.clone(), bl_function.args.get(i).unwrap().arg_var_type.clone()) {
                                (
                                    CgExpression::Literal(Literal::Number(_)),
                                    VariableType::I8 | VariableType::I16 | VariableType::I32
                                ) => true,
                                (
                                    CgExpression::StackVariableIdentifier(identifier),
                                    _
                                ) => {
                                    match self.borrow_stack_variable_with_sf_index(stack_frame, identifier) {
                                        Some(var) => {
                                            var.variable_type == bl_function.args.get(i).unwrap().arg_var_type
                                        },
                                        None => false
                                    }
                                },
                                _ => false
                            };

                            if !valid {
                                panic!();
                            }

                            cg_args.push(cg_expression);

                            i += 1;
                        }

                        self.add_cg_statement_to_stack_frame(stack_frame, CgStatement { statement_type: CgStatementType::BuiltInFunction(CgBuiltInFunctions::BranchLinked(CgBranchLinked{function_name: branch_linked.function_name, args: cg_args}))});
                    },
                    BuiltInFunctionsAst::Assembly(asm_expression) => {
                        let asm_code : String = match *asm_expression {
                            Expression::Literal(Literal::String(asm_code)) => asm_code,
                            Expression::BuiltInFunction(BuiltInFunctionsAst::Format(format)) => {
                                format.parse()
                            },
                            _ => {panic!("Invalid shit given to asm func");}
                        };

                        self.add_cg_statement_to_stack_frame(stack_frame, CgStatement{ statement_type: CgStatementType::BuiltInFunction(CgBuiltInFunctions::Assembly(asm_code))});
                    }
                    _ => {}
                }
            }
            _ => ()
        };

        return Ok(None);
    }

    pub fn expression_to_cg(&mut self, stack_frame : usize, expression : Expression) -> CgExpression {
        match expression {
            Expression::Literal(literal) => return CgExpression::Literal(literal),
            Expression::Identifier(Identifiers::Identifier(identifier)) => {
                if let Some(variable) = self.get_stack_frame_by_index(stack_frame).variables.get(&identifier) {
                    return CgExpression::StackVariableIdentifier(identifier);
                }

                panic!()
            },
            _ => panic!()
        }
    }

    pub fn process_stack_frame(&mut self, stack_frame : usize) -> () {
        for statement in self.get_stack_frame_by_index(stack_frame).statements.clone().iter() {
            let analyzed_statement_err = self.process_statement(statement, stack_frame);

            if let Err(err) = analyzed_statement_err {
                panic!("{:?}", err);
            }
        }

        return;
    }

    pub fn process_stack_frame_and_children(&mut self, stack_frame_index : usize) -> () {
        self.traverse_stack_frame_children(stack_frame_index);
    }

    pub fn process_all_functions(&mut self) -> () {
        for (function_name, function) in self.program_data.functions.clone().iter() {
            self.process_stack_frame_and_children(function.first_stack_frame.clone());
        }
    }

    pub fn add_cg_statement_to_stack_frame(&mut self, stack_frame : usize, statement : CgStatement) -> () {
        self.get_stack_frame_by_index_mut(stack_frame).cg_statements.push(statement);
    }

    pub fn traverse_stack_frame_children(&mut self, stack_frame_index : usize) -> () {
        let children = self.get_stack_frame_by_index(stack_frame_index).children.clone();

        for child in children.iter() {
            self.traverse_stack_frame_children(child.clone());
        }

        self.process_stack_frame(stack_frame_index);
    }

    pub fn borrow_stack_variable_with_sf_index(&self, stack_frame : usize, variable_name : String) -> Option<&'_ StackVariable> {
        return self.get_stack_frame_by_index(stack_frame).variables.get(&variable_name);
    }

    pub fn get_stack_frame_by_index(&self, index : usize) -> &'_ StackFrame {
        return self.program_data.stack_frames.get(index).unwrap();
    }
    
    pub fn get_stack_frame_by_index_mut(&mut self, index : usize) -> &'_ mut StackFrame {
        return self.program_data.stack_frames.get_mut(index).unwrap();
    }
}
