use std::collections::HashMap;
use crate::datatypes::{ast_statements::{CgBuiltInFunctions, CgStatement, CgStatementType, Expression, Literal, Statement, Statements}, stack_frame::StackFrame};

pub struct CodeGenerator<'a> {
    stack_frames: &'a Vec<StackFrame>,
    functions: &'a HashMap<String, usize>
}

impl<'a> CodeGenerator<'a> {
    pub fn new(stack_frames : &'a Vec<StackFrame>, functions : &'a HashMap<String, usize>) -> Self {
        return Self{stack_frames, functions};
    }

    pub fn generate_statement(&mut self, statement : &CgStatement) -> Result<String, String> {
        match statement.statement_type.clone() {
            CgStatementType::VariableInitialization(var_init) => {
                let stack_frame_borrow = self.get_stack_frame_by_index(var_init.stack_frame);

                let offset = stack_frame_borrow.variables.get(&var_init.var_name).unwrap().offset;

                match var_init.init_value {
                    Expression::Literal(literal) => {
                        match literal {
                            Literal::Number(num_literal) => {
                                return Ok(format!("mov x10, #{}\nstr x10, [x29, #{}]\n", num_literal, offset));
                            },
                            _ => {
                                return Err("Not supported Literal".to_string());
                            }
                        }
                    },
                }
            },
            CgStatementType::BuiltInFunction(built_in_function) => {
                match built_in_function {
                    CgBuiltInFunctions::Assembly(assembly_code) => {
                        return Ok(assembly_code);
                    }
                }
            }
        };
    }

    pub fn align_memory(&self, mem : usize, alignment : usize) -> usize {
        return (mem + (alignment - 1)) & !(alignment - 1)
    }

    pub fn initialize_stack_frame(&mut self, stack_frame : usize) -> String {
        let stack_frame_borrow = self.get_stack_frame_by_index(stack_frame);

        return format!("stp x29, x30, [sp, #-16]!\nmov x29, sp\nsub sp, sp, #{}\n", self.align_memory(stack_frame_borrow.stack_mem_allocated, 16));
    }

    pub fn return_stack_frame(&mut self, stack_frame : usize) -> String {
        let stack_frame_borrow = self.get_stack_frame_by_index(stack_frame);

        return format!("add sp, sp, #{}\nldp x29, x30, [sp], #16\nret\n", self.align_memory(stack_frame_borrow.stack_mem_allocated, 16));
    }

    pub fn process_stack_frame(&mut self, stack_frame : usize) -> String {
        let mut result = String::new();

        result.push_str(&self.initialize_stack_frame(stack_frame));

        for statement in self.get_stack_frame_by_index(stack_frame).statements.clone().iter() {
            let analyzed_statement_err = self.generate_statement(statement);

            match analyzed_statement_err {
                Err(err) => panic!("{:?}", err),
                Ok(asm_code) => {
                    result.push_str(&asm_code);
                }
            }
        }

        result.push_str(&self.return_stack_frame(stack_frame));

        return result;
    }

    pub fn process_stack_frame_and_children(&mut self, stack_frame_index : usize) -> String {
        let compiled_code = self.traverse_stack_frame_children(stack_frame_index);

        return compiled_code;
    }

    pub fn process_all_functions(&mut self) -> String {
        let mut result = String::new();

        for (function_name, stack_frame) in self.functions {
            let function_start = format!("_{}:\n", function_name);
            result.push_str(&function_start);

            result.push_str(&self.process_stack_frame_and_children(stack_frame.clone()));
        }

        return result;
    }

    pub fn traverse_stack_frame_children(&mut self, stack_frame_index : usize) -> String {
        let mut result = String::new();

        let children = self.get_stack_frame_by_index(stack_frame_index).children.clone();

        let compiled_code = self.process_stack_frame(stack_frame_index);

        result.push_str(&compiled_code);

        for child in children.iter() {
            let child_compiled = self.traverse_stack_frame_children(child.clone());

            result.push_str(&child_compiled);
        }

        return compiled_code;
    }

    pub fn get_stack_frame_by_index(&self, index : usize) -> &'_ StackFrame {
        return self.stack_frames.get(index).unwrap();
    }
}
