use crate::datatypes::{ast_statements::{CgBuiltInFunctions, CgStatement, CgStatementType, Expression, Literal, Statement, Statements, VariableType}, program_data::ProgramData, stack_frame::StackFrame};

pub struct CodeGenerator<'a> {
    program_data: &'a mut ProgramData,
}

impl<'a> CodeGenerator<'a> {
    pub fn new(program_data : &'a mut ProgramData) -> Self {
        return Self{program_data};
    }

    pub fn generate_statement(&mut self, statement : &CgStatement) -> Result<String, String> {
        match statement.statement_type.clone() {
            CgStatementType::VariableInitialization(var_init) => {
                let stack_frame_borrow = self.get_stack_frame_by_index(var_init.stack_frame);

                let variable = stack_frame_borrow.variables.get(&var_init.var_name).unwrap();

                match var_init.init_value {
                    Expression::Literal(literal) => {
                        match literal {
                            Literal::Number(num_literal) => {
                                match variable.variable_type {
                                    VariableType::I32 => {
                                        return Ok(format!("mov w10, #{}\nstr w10, [x29, #-{}]\n", num_literal, stack_frame_borrow.stack_mem_allocated - variable.offset));
                                    },
                                    VariableType::I16 => {
                                        return Ok(format!("mov w10, #{}\nstrh w10, [x29, #-{}]\n", num_literal, stack_frame_borrow.stack_mem_allocated - variable.offset));
                                    },
                                    VariableType::I8 => {
                                        return Ok(format!("mov w10, #{}\nstrb w10, [x29, #-{}]\n", num_literal, stack_frame_borrow.stack_mem_allocated - variable.offset));
                                    },
                                    _ => {
                                        return Err("Error init var".to_string());
                                    }
                                }
                            },
                            _ => {
                                return Err("Not supported Literal".to_string());
                            }
                        }
                    },
                    _ => unimplemented!()
                }
            },
            CgStatementType::BuiltInFunction(built_in_function) => {
                match built_in_function {
                    CgBuiltInFunctions::BranchLinked(branch_linked) => {
                        return Ok(format!("bl _{}\n", branch_linked.function_name));
                    },
                    CgBuiltInFunctions::Assembly(assembly_code) => {
                        return Ok(assembly_code);
                    }
                }
            }
        };
    }

    pub fn align_memory(&self, mem : usize, alignment : usize) -> usize {
        return (mem + (alignment - 1)) & !(alignment - 1);
    }

    pub fn initialize_stack_frame(&mut self, stack_frame : usize) -> String {
        let mem = self.get_stack_frame_by_index(stack_frame).stack_mem_allocated.clone();

        let aligned_mem = self.align_memory(mem, 16);

        self.get_stack_frame_by_index_mut(stack_frame).stack_mem_allocated = aligned_mem;

        return format!("stp x29, x30, [sp, #-16]!\nmov x29, sp\nsub sp, sp, #{}\n", aligned_mem);
    }

    pub fn return_stack_frame(&mut self, stack_frame : usize) -> String {
        let stack_frame_borrow = self.get_stack_frame_by_index(stack_frame);

        return format!("add sp, sp, #{}\nldp x29, x30, [sp], #16\nret\n", stack_frame_borrow.stack_mem_allocated);
    }

    pub fn process_stack_frame(&mut self, stack_frame : usize) -> String {
        let mut result = String::new();

        result.push_str(&self.initialize_stack_frame(stack_frame));

        for statement in self.get_stack_frame_by_index(stack_frame).cg_statements.clone().iter() {
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

        for (function_name, stack_frame) in self.program_data.functions.clone() {
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
        return self.program_data.stack_frames.get(index).unwrap();
    }

    pub fn get_stack_frame_by_index_mut(&mut self, index : usize) -> &'_ mut StackFrame {
        return self.program_data.stack_frames.get_mut(index).unwrap();
    }
}
