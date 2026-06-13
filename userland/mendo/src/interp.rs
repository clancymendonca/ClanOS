//! Tree-walking interpreter for Mendo.

use crate::parser::{BinOp, Expr, Program, Stmt};
use crate::syscall;

const MAX_VARS: usize = 8;
const STDOUT_FD: u64 = 1;

#[derive(Debug, Clone, Copy)]
struct VarSlot<'a> {
    name: Option<&'a str>,
    value: i64,
}

struct Env<'a> {
    slots: [VarSlot<'a>; MAX_VARS],
}

impl<'a> Env<'a> {
    const fn new() -> Self {
        Self {
            slots: [VarSlot {
                name: None,
                value: 0,
            }; MAX_VARS],
        }
    }

    fn get(&self, name: &str) -> i64 {
        for slot in &self.slots {
            if slot.name == Some(name) {
                return slot.value;
            }
        }
        0
    }

    fn set(&mut self, name: &'a str, value: i64) {
        for slot in &mut self.slots {
            if slot.name == Some(name) {
                slot.value = value;
                return;
            }
        }
        for slot in &mut self.slots {
            if slot.name.is_none() {
                slot.name = Some(name);
                slot.value = value;
                return;
            }
        }
    }
}

fn write_bytes(bytes: &[u8]) {
    let mut offset = 0usize;
    while offset < bytes.len() {
        let chunk = core::cmp::min(32, bytes.len() - offset);
        let _ = syscall::sys_write(STDOUT_FD, bytes[offset..].as_ptr(), chunk);
        offset += chunk;
    }
}

fn write_i64(mut n: i64) {
    let mut buf = [0u8; 22];
    if n == 0 {
        write_bytes(b"0");
        return;
    }
    let negative = n < 0;
    if negative {
        n = n.saturating_neg();
    }
    let mut len = 0usize;
    while n > 0 && len < buf.len() {
        buf[len] = b'0' + (n % 10) as u8;
        n /= 10;
        len += 1;
    }
    if negative {
        write_bytes(b"-");
    }
    while len > 0 {
        len -= 1;
        write_bytes(&buf[len..len + 1]);
    }
}

fn eval_expr<'a>(program: &Program<'a>, env: &Env<'a>, id: u8) -> i64 {
    match program.exprs[id as usize] {
        Expr::Int(v) => v,
        Expr::Var(name) => env.get(name),
        Expr::Bin(op, left, right) => {
            let l = eval_expr(program, env, left);
            let r = eval_expr(program, env, right);
            match op {
                BinOp::Add => l.saturating_add(r),
                BinOp::Sub => l.saturating_sub(r),
                BinOp::Mul => l.saturating_mul(r),
                BinOp::Div => {
                    if r == 0 {
                        0
                    } else {
                        l / r
                    }
                }
                BinOp::Eq => i64::from(l == r),
                BinOp::Ne => i64::from(l != r),
                BinOp::Lt => i64::from(l < r),
                BinOp::Le => i64::from(l <= r),
                BinOp::Gt => i64::from(l > r),
                BinOp::Ge => i64::from(l >= r),
            }
        }
    }
}

fn run_range<'a>(program: &Program<'a>, env: &mut Env<'a>, start: u16, len: u16) {
    let end = start.saturating_add(len) as usize;
    let mut pc = start as usize;
    while pc < end {
        pc = run_stmt(program, env, pc);
    }
}

fn run_stmt<'a>(program: &Program<'a>, env: &mut Env<'a>, pc: usize) -> usize {
    match program.stmts[pc] {
        Stmt::PrintStr(text) => {
            write_bytes(text.as_bytes());
            pc + 1
        }
        Stmt::PrintExpr(expr) => {
            write_i64(eval_expr(program, env, expr));
            write_bytes(b"\n");
            pc + 1
        }
        Stmt::Let(name, expr) => {
            env.set(name, eval_expr(program, env, expr));
            pc + 1
        }
        Stmt::Assign(name, expr) => {
            env.set(name, eval_expr(program, env, expr));
            pc + 1
        }
        Stmt::If {
            cond,
            then_start,
            then_len,
            else_start,
            else_len,
        } => {
            if eval_expr(program, env, cond) != 0 {
                run_range(program, env, then_start, then_len);
            } else if else_len > 0 {
                run_range(program, env, else_start, else_len);
            }
            pc + 1
        }
        Stmt::While {
            cond,
            body_start,
            body_len,
        } => {
            while eval_expr(program, env, cond) != 0 {
                run_range(program, env, body_start, body_len);
            }
            pc + 1
        }
    }
}

pub fn run<'a>(program: &Program<'a>) {
    let mut env = Env::new();
    let mut pc = 0usize;
    while pc < program.stmt_count {
        pc = run_stmt(program, &mut env, pc);
    }
}
