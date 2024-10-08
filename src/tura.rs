use std::env;
use std::fmt::Write;
use std::fs;
use std::iter::Peekable;
use std::process::ExitCode;
use std::result;

type Result<T> = result::Result<T, ()>;
#[derive(Debug)]
enum Step {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Symbol<'a> {
    name: &'a str,
}

#[derive(Debug)]
struct Rule<'a> {
    state: Symbol<'a>,
    read: Symbol<'a>,
    write: Symbol<'a>,
    next: Symbol<'a>,
    step: Step,
}

#[derive(Debug)]
struct Machine<'a> {
    state: Symbol<'a>,
    tape: Vec<Symbol<'a>>,
    tape_default: Symbol<'a>,
    head: usize,
    halt: bool,
}

impl<'a> Machine<'a> {
    fn next(&mut self, rules: &[Rule<'a>]) -> Result<()> {
        for rule in rules {
            if rule.state == self.state && rule.read == self.tape[self.head] {
                self.tape[self.head].name = rule.write.name;

                match rule.step {
                    Step::Left => {
                        if self.head == 0 {
                            eprintln!("ERROR: head moved out of bounds");
                            return Err(());
                        }

                        self.head -= 1;
                    }
                    Step::Right => {
                        self.head += 1;
                        if self.head >= self.tape.len() {
                            self.tape.push(self.tape_default.clone());
                        }
                    }
                }
                self.state.name = rule.next.name;
                self.halt = false;
                break;
            }
        }
        Ok(())
    }

    fn print(&self) {
        let mut buffer = String::new();
        let _ = write!(&mut buffer, "{state}: ", state = self.state.name);
        let mut head = 0;
        for (i, symbol) in self.tape.iter().enumerate() {
            if i == self.head {
                head = buffer.len();
            }
            let _ = write!(&mut buffer, "{name} ", name = symbol.name);
        }
        println!("{buffer}");

        for _ in 0..head {
            print!(" ");
        }
        println!("^");
    }
}

fn parse_symbol<'a>(lexer: &mut impl Iterator<Item = &'a str>) -> Result<Symbol<'a>> {
    if let Some(name) = lexer.next() {
        Ok(Symbol { name })
    } else {
        eprintln!("ERROR: expected symbol, but reached end of input");
        Err(())
    }
}

fn parse_step<'a>(lexer: &mut impl Iterator<Item = &'a str>) -> Result<Step> {
    let symbol = parse_symbol(lexer)?;
    match symbol.name {
        "L" => Ok(Step::Left),
        "R" => Ok(Step::Right),
        name => {
            eprintln!("ERROR: expected R or L, but got {name}");
            Err(())
        }
    }
}

fn parse_rule<'a>(lexer: &mut impl Iterator<Item = &'a str>) -> Result<Rule<'a>> {
    let state = parse_symbol(lexer)?;
    let read = parse_symbol(lexer)?;
    let write = parse_symbol(lexer)?;
    let step = parse_step(lexer)?;
    let next = parse_symbol(lexer)?;

    Ok(Rule {
        state,
        read,
        write,
        next,
        step,
    })
}

fn parse_rules<'a>(lexer: &mut Peekable<impl Iterator<Item = &'a str>>) -> Result<Vec<Rule<'a>>> {
    let mut rules = vec![];
    while lexer.peek().is_some() {
        rules.push(parse_rule(lexer)?);
    }
    Ok(rules)
}

fn parse_tape<'a>(lexer: &mut Peekable<impl Iterator<Item = &'a str>>) -> Result<Vec<Symbol<'a>>> {
    let mut symbols = vec![];
    while lexer.peek().is_some() {
        symbols.push(parse_symbol(lexer)?);
    }
    Ok(symbols)
}

fn start() -> Result<()> {
    let mut args = env::args();
    let program = args.next().expect("ERROR: could not get program name");

    let tura_path;
    if let Some(path) = args.next() {
        tura_path = path;
    } else {
        eprintln!("Usage: {program} <source.tura>");
        eprintln!("ERROR: expected source file");
        return Err(());
    }

    let tape_path;
    if let Some(path) = args.next() {
        tape_path = path;
    } else {
        eprintln!("Usage: {program} {tura_path} <input.tape>");
        eprintln!("ERROR: expected tape file");
        return Err(());
    }

    let tura_source = fs::read_to_string(&tura_path).map_err(|err| {
        eprintln!("ERROR: could not read file {tura_path}: {err}");
    })?;

    let rules = parse_rules(
        &mut tura_source
            .split(&[' ', '\n'])
            .filter(|token| token.len() > 0)
            .peekable(),
    )?;

    let tape_source = fs::read_to_string(&tape_path).map_err(|err| {
        eprintln!("ERROR: could not read file {tape_path}: {err}");
    })?;

    let tape = parse_tape(
        &mut tape_source
            .split(&[' ', '\n'])
            .filter(|token| token.len() > 0)
            .peekable(),
    )?;

    let tape_default;
    if let Some(symbol) = tape.last().cloned() {
        tape_default = symbol;
    } else {
        eprintln!("ERROR: tape is empty");
        return Err(());
    }

    let mut machine = Machine {
        state: Symbol { name: "Inc" },
        tape,
        tape_default,
        head: 0,
        halt: false,
    };

    while !machine.halt {
        machine.print();
        machine.halt = true;
        machine.next(&rules)?;
    }

    Ok(())
}

fn main() -> ExitCode {
    match start() {
        Ok(()) => ExitCode::SUCCESS,
        Err(()) => ExitCode::FAILURE,
    }
}
