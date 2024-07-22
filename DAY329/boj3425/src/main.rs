use std::fmt::Write;
use std::io;

#[derive(Clone, Copy)]
enum Cmd {
    Nnm(i64),
    Pop,
    Inv,
    Dup,
    Swp,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

fn go_stack(v: i64, cmd_list: &Vec<Cmd>) -> Option<i64> {
    let mut stack = Vec::new();

    stack.push(v);

    for &cmd in cmd_list {
        match cmd {
            Cmd::Nnm(x) => {
                stack.push(x);
            }
            Cmd::Pop => {
                stack.pop()?;
            }
            Cmd::Inv => {
                let x = stack.pop()?;
                stack.push(-x);
            }
            Cmd::Dup => {
                let x = stack.last()?;
                stack.push(*x);
            }
            Cmd::Swp => {
                let f = stack.pop()?;
                let s = stack.pop()?;
                stack.push(f);
                stack.push(s);
            }
            Cmd::Add => {
                let f = stack.pop()?;
                let s = stack.pop()?;
                stack.push(f + s);
            }
            Cmd::Sub => {
                let f = stack.pop()?;
                let s = stack.pop()?;
                stack.push(s - f);
            }
            Cmd::Mul => {
                let f = stack.pop()?;
                let s = stack.pop()?;
                stack.push(f * s);
            }
            Cmd::Div => {
                let f = stack.pop()?;
                let s = stack.pop()?;
                stack.push(s.checked_div(f)?);
            }
            Cmd::Mod => {
                let f = stack.pop()?;
                let s = stack.pop()?;
                stack.push(s.checked_rem(f)?);
            }
        }

        if let Some(&x) = stack.last() {
            if x.abs() > 1_000_000_000 { None? }
        }
    }

    if stack.len() == 1 {
        stack.pop()
    } else {
        None
    }
}

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    'a: loop {
        let mut cmd_list = Vec::new();

        loop {
            match next!() {
                "QUIT" => break 'a,
                "END"  => break,
                "NUM"  => cmd_list.push(Cmd::Nnm(next!(i64))),
                "POP"  => cmd_list.push(Cmd::Pop),
                "INV"  => cmd_list.push(Cmd::Inv),
                "DUP"  => cmd_list.push(Cmd::Dup),
                "SWP"  => cmd_list.push(Cmd::Swp),
                "ADD"  => cmd_list.push(Cmd::Add),
                "SUB"  => cmd_list.push(Cmd::Sub),
                "MUL"  => cmd_list.push(Cmd::Mul),
                "DIV"  => cmd_list.push(Cmd::Div),
                "MOD"  => cmd_list.push(Cmd::Mod),
                _ => unreachable!(),
            }
        }

        let n = next!(u16);
        for v in (0..n).map(|_| next!(i64)) {
            let _ = match go_stack(v, &cmd_list) {
                Some(x) => writeln!(output, "{x}"),
                None    => writeln!(output, "ERROR"),
            };
        }

        output.push('\n');
    }

    print!("{output}");
}