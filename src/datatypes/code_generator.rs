use std::fmt::format;

use crate::datatypes::{assembly_instructions::asm::*, ast_statements::{CgBuiltInFunctions, CgExpression, CgStatement, CgStatementType, Expression, Literal, MemoryLocationsAst, VariableType}, program_data::ProgramData, stack_frame::{StackFrame, StackVariable}};

pub struct CodeGenerator<'a> {
    program_data: &'a mut ProgramData,
}

impl<'a> CodeGenerator<'a> {
    pub fn new(program_data : &'a mut ProgramData) -> Self {
        return Self{program_data};
    }

    pub fn generate_statement(&mut self, statement : &CgStatement, stack_frame : usize) -> Result<String, String> {
        match statement.statement_type.clone() {
            CgStatementType::VariableInitialization(var_init) => {
                let stack_frame_borrow = self.get_stack_frame_by_index(var_init.stack_frame);

                let variable = stack_frame_borrow.variables.get(&var_init.var_name).unwrap();

                return Ok(self.init_stack_var(variable.variable_type.clone(), var_init.init_value, stack_frame_borrow.stack_mem_allocated - variable.offset - variable.variable_size, REG_FRAME_STACK_PTR));

                /*match var_init.init_value {
                    Expression::Literal(literal) => {
                        match literal {
                            Literal::Number(num_literal) => {
                                match variable.variable_type {
                                    VariableType::I32 => {
                                        return Ok(format!("mov w10, #{}\nstr w10, [x29, #-{}]\n", num_literal, stack_frame_borrow.stack_mem_allocated - variable.offset - variable.variable_size));
                                    },
                                    VariableType::I16 => {
                                        return Ok(format!("mov w10, #{}\nstrh w10, [x29, #-{}]\n", num_literal, stack_frame_borrow.stack_mem_allocated - variable.offset - variable.variable_size));
                                    },
                                    VariableType::I8 => {
                                        return Ok(format!("mov w10, #{}\nstrb w10, [x29, #-{}]\n", num_literal, stack_frame_borrow.stack_mem_allocated - variable.offset - variable.variable_size));
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
                }*/
            },
            CgStatementType::BuiltInFunction(built_in_function) => {
                match built_in_function {
                    CgBuiltInFunctions::BranchLinked(branch_linked) => {
                        let mut result = String::new();

                        let function_borrow = self.program_data.functions.get(&branch_linked.function_name).unwrap();

                        let mut arg_mem = 0;
                        for arg in function_borrow.args.clone() {
                            if arg.memory_location != MemoryLocationsAst::Stack {
                                continue;
                            }

                            arg_mem += arg.arg_var_type.get_variable_size();
                        }

                        let aligned_memory = self.align_memory(arg_mem, 16);

                        result.push_str(&format!("sub {}, {}, #{}\n", REG_STACK_PTR, REG_STACK_PTR, aligned_memory));

                        let mut arg_stack_offset = 0;

                        for i in 0..function_borrow.args.len() {
                            let arg_provided = branch_linked.args.get(i).unwrap();
                            let arg_expecting = function_borrow.args.get(i).unwrap();

                            match arg_expecting.memory_location.clone() {
                                MemoryLocationsAst::Register(register) => {
                                    match arg_provided {
                                        CgExpression::Literal(Literal::Number(num)) => {
                                            result.push_str(&format!("mov {}, #{}\n", register, num));
                                        },
                                        CgExpression::StackVariableIdentifier(identifier) => {
                                            self.get_stack_variable(stack_frame, identifier);

                                            result.push_str(&format!("mov {}, #{}", register, 10));
                                        },
                                        _ => todo!()
                                    }
                                },
                                MemoryLocationsAst::Stack => {
                                    let var_size = arg_expecting.arg_var_type.get_variable_size();

                                    arg_stack_offset += var_size;

                                    result.push_str(&self.init_stack_var(arg_expecting.arg_var_type.clone(), arg_provided.clone(), aligned_memory - arg_stack_offset, REG_STACK_PTR));
                                }
                                _ => todo!()
                            }
                        }

                        result.push_str(&format!("bl _{}\nadd {}, {}, #{}\n", branch_linked.function_name, REG_STACK_PTR, REG_STACK_PTR, aligned_memory));

                        return Ok(result);
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

    // ASSEMBLY INSTRUCTION WRAPPERS
    
    pub fn init_stack_var(&self, variable_type : VariableType, initial_value : CgExpression, var_stack_loc : usize, stack_ptr : &str) -> String {
        let ret_str = match (variable_type, initial_value) {
            (VariableType::I32, CgExpression::Literal(Literal::Number(num))) => format!("mov {}, {}\n{} {}, [{}, #-{}]\n", REG_TEMP_STR_32, num, INST_STR_32, REG_TEMP_STR_32, stack_ptr, var_stack_loc),
            (VariableType::I16, CgExpression::Literal(Literal::Number(num))) => format!("mov {}, {}\n{} {}, [{}, #-{}]\n", REG_TEMP_STR_16, num, INST_STR_16, REG_TEMP_STR_16, stack_ptr, var_stack_loc),
            (VariableType::I8, CgExpression::Literal(Literal::Number(num))) => format!("mov {}, {}\n{} {}, [{}, #-{}]\n", REG_TEMP_STR_8, num, INST_STR_8, REG_TEMP_STR_8, stack_ptr, var_stack_loc),
            _ => unreachable!()
        };

        return ret_str;
    }

    // END

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
            let analyzed_statement_err = self.generate_statement(statement, stack_frame);

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

            result.push_str(&self.process_stack_frame_and_children(stack_frame.first_stack_frame.clone()));
        }

        return result;
    }

    pub fn get_stack_variable(&mut self, stack_frame : usize, var_name : &String) -> &'_ StackVariable {
        return self.get_stack_frame_by_index(stack_frame).variables.get(var_name).unwrap();
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
