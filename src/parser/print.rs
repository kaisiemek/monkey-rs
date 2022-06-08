use super::ast::Program;

#[allow(unused)]
pub fn program_to_string(program: Program) -> String {
    let mut program_str = String::new();
    for stmt in program {
        program_str.push_str(&stmt.to_string());
    }

    program_str
}
