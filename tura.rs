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

#[derive(Debug, PartialEq)]
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
    let mut rules = Vec::new();
    while lexer.peek().is_some() {
        rules.push(parse_rule(lexer)?);
    }
    Ok(rules)
}

fn start() -> Result<()> {
    let mut args = env::args();
    let program = args.next().expect("ERROR: could not get program name");

    let source_path;
    if let Some(path) = args.next() {
        source_path = path;
    } else {
        eprintln!("Usage: {program} <source.tura>");
        eprintln!("ERROR: expected source file");
        return Err(());
    }

    let source = fs::read_to_string(&source_path).map_err(|err| {
        eprintln!("ERROR: could not read file {source_path}: {err}");
    })?;

    let mut lexer = source
        .split(&[' ', '\n'])
        .filter(|token| token.len() > 0)
        .peekable();
    let rules = parse_rules(&mut lexer)?;

    let mut machine = Machine {
        state: Symbol { name: "Inc" },
        tape: vec![
            Symbol { name: "1" },
            Symbol { name: "1" },
            Symbol { name: "0" },
            Symbol { name: "1" },
        ],
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
