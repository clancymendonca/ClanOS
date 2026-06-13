//! Mendo — minimal ring-3 scripting language for Clan OS.

#![no_std]
#![no_main]

mod interp;
mod lexer;
mod parser;
mod syscall;

use lexer::{tokenize, LexError, TokenBuf};
use parser::{parse, ParseError, Program};

const SCRIPT: &str = r#"
print "hello from mendo\n";
let x = 1;
if x == 1 {
  print "x is one\n";
}
while x < 3 {
  print x;
  x = x + 1;
}
"#;

fn run_script() -> u64 {
    let mut tokens = TokenBuf::new();
    if tokenize(SCRIPT, &mut tokens).is_err() {
        return 2;
    }
    let mut program = Program::new();
    if parse(&tokens, &mut program).is_err() {
        return 3;
    }
    interp::run(&program);
    0
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let code = run_script();
    syscall::sys_exit(code);
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    syscall::sys_exit(99);
}

#[allow(dead_code)]
fn _lex_errors() {
    let _ = LexError::UnexpectedChar;
    let _ = ParseError::UnexpectedToken;
}
