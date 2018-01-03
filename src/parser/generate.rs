use super::ast::*;

pub fn generate(prog: Program) -> String {
    let asm: Vec<String> = match prog {
        Program { func, globals } => {
            let text: Vec<String> = func.into_iter().map(|a| gen_function(a).join("\n") ).collect();
            let data : Vec<String> = globals.into_iter().map(|g| format!("_{0}: .word 0", g)).collect();

            vec![
                text,
                s(".data"),
                data,
            ].concat()
        },
    };

    asm.join("\n")
}

fn gen_function(fun: Function) -> Vec<String> {
    match fun {
        Function { name, statements } => {
            let s: Vec<Vec<String>> = statements.into_iter().map(|statement| gen_statement(statement)).collect();
            vec![
                vec![
                    format!(".global _{0}", name),
                    format!("_{0}:", name)
                ],
                s.concat(),
            ].concat()
        }
    }
}

fn gen_statement(stat: Statement) -> Vec<String> {
    match stat {
        Statement::Return(exp) => vec![
            gen_expression(exp),
            s("ret")
        ].concat(),
        Statement::Assign(name, exp) => vec![
            gen_expression(exp),
            vec![format!("movl %eax, _{}", name)]
        ].concat(),
    }
}

fn gen_expression(exp: Expression) -> Vec<String> {
    match exp {
        Expression::Int(val) => vec![format!("movl ${}, %eax", val)],
        Expression::UnOp(op, exp) => {
            let asm = match op {
                UnOp::Negation => s("neg %eax"),
                UnOp::BitComp => s("not %eax"),
                UnOp::LogicalNeg => sa(&[
                    "cmpl $0, %eax",
                    "movl $0, %eax",
                    "sete %al"
                ]),
            };
            vec![gen_expression(*exp), asm].concat()
        },
        Expression::BinOp(op, exp1, exp2) => {
            match op {
                BinOp::Addition => vec![
                    gen_expression(*exp1),
                    s("push %eax"),
                    gen_expression(*exp2),
                    s("pop %ecx"),
                    s("addl %ecx, %eax"),
                ].concat(),
                BinOp::Subtraction => vec![
                    gen_expression(*exp2),
                    s("push %eax"),
                    gen_expression(*exp1),
                    s("pop %ecx"),
                    s("subl %ecx, %eax"),
                ].concat(),
                BinOp::Multiplication => vec![
                    gen_expression(*exp1),
                    s("push %eax"),
                    gen_expression(*exp2),
                    s("pop %ecx"),
                    s("imul %ecx, %eax"),
                ].concat(),
                BinOp::Division => vec![
                    gen_expression(*exp2),
                    s("push %eax"),
                    gen_expression(*exp1),
                    s("pop %ecx"),
                    s("xor %edx, %edx"),
                    s("idivl %ecx"),
                ].concat(),
                BinOp::Modulus => vec![
                    gen_expression(*exp2),
                    s("push %eax"),
                    gen_expression(*exp1),
                    s("pop %ecx"),
                    s("xor %edx, %edx"),
                    s("idivl %ecx"),
                    s("movl %edx, %eax"),
                ].concat(),
                BinOp::Equal => vec![
                    gen_expression(*exp1),
                    s("push %eax"),
                    gen_expression(*exp2),
                    s("pop %ecx"),
                    s("cmpl %ecx, %eax"),
                    s("sete %al"),
                ].concat(),
                BinOp::NotEqual => vec![
                    gen_expression(*exp1),
                    s("push %eax"),
                    gen_expression(*exp2),
                    s("pop %ecx"),
                    s("cmpl %ecx, %eax"),
                    s("setne %al"),
                ].concat(),
                BinOp::LessThan => vec![
                    gen_expression(*exp2),
                    s("push %eax"),
                    gen_expression(*exp1),
                    s("pop %ecx"),
                    s("cmpl %ecx, %eax"),
                    s("setl %al"),
                ].concat(),
                BinOp::LessThanOrEqual => vec![
                    gen_expression(*exp2),
                    s("push %eax"),
                    gen_expression(*exp1),
                    s("pop %ecx"),
                    s("cmpl %ecx, %eax"),
                    s("setle %al"),
                ].concat(),
                BinOp::GreaterThan => vec![
                    gen_expression(*exp2),
                    s("push %eax"),
                    gen_expression(*exp1),
                    s("pop %ecx"),
                    s("cmpl %ecx, %eax"),
                    s("setg %al"),
                ].concat(),
                BinOp::GreaterThanOrEqual => vec![
                    gen_expression(*exp2),
                    s("push %eax"),
                    gen_expression(*exp1),
                    s("pop %ecx"),
                    s("cmpl %ecx, %eax"),
                    s("setge %al"),
                ].concat(),
                BinOp::Or => vec![
                    gen_expression(*exp1),
                    s("push %eax"),
                    gen_expression(*exp2),
                    s("pop %ecx"),
                    s("orl %ecx, %eax"),
                    s("setne %al"),
                ].concat(),
                BinOp::And => vec![
                    gen_expression(*exp1),
                    s("push %eax"),
                    gen_expression(*exp2),
                    s("pop %ecx"),
                    s("cmpl $0, %ecx"),
                    s("setne %cl"),
                    s("cmpl $0, %eax"),
                    s("setne %al"),
                    s("andb %cl, %al"),
                ].concat(),
                BinOp::BitwiseLeft => vec![
                    gen_expression(*exp2),
                    s("push %eax"),
                    gen_expression(*exp1),
                    s("pop %ecx"),
                    s("shll %cl, %eax")
                ].concat(),
                BinOp::BitwiseRight => vec![
                    gen_expression(*exp2),
                    s("push %eax"),
                    gen_expression(*exp1),
                    s("pop %ecx"),
                    s("shrl %cl, %eax")
                ].concat(),
                BinOp::BitwiseAnd => vec![
                    gen_expression(*exp1),
                    s("push %eax"),
                    gen_expression(*exp2),
                    s("pop %ecx"),
                    s("andl %ecx, %eax")
                ].concat(),
                BinOp::BitwiseOr => vec![
                    gen_expression(*exp1),
                    s("push %eax"),
                    gen_expression(*exp2),
                    s("pop %ecx"),
                    s("orl %ecx, %eax"),
                ].concat(),
                BinOp::BitwiseXor => vec![
                    gen_expression(*exp1),
                    s("push %eax"),
                    gen_expression(*exp2),
                    s("pop %ecx"),
                    s("xorl %ecx, %eax"),
                ].concat(),
            }
        },
        Expression::Variable(name) => {
            vec![format!("movl _{}, %eax", name)]
        }
        Expression::FunctionCall(name) => {
            vec![format!("call _{}", name)]
        }
    }
}

fn s(string: &str) -> Vec<String> {
    vec![string.to_string()]
}

fn sa(string: &[&str]) -> Vec<String> {
    string.iter().map({|s| s.to_string() }).collect()
}
