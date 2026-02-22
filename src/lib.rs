use std::vec::Vec;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    String,
    Integer,
    Any,
    // Number,
}

#[derive(Debug)]
pub enum Error {
    UnknownArgument(String),
    ExpectedParameter {
        argument: String,
        parameter: String,
    }
}

#[derive(Clone, Debug)]
pub struct Argument {
    name: &'static str,
    aliases: Vec<&'static str>,
    pub params: Vec<Parameter>,
    pub optional: bool,
    pub help: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Parameter {
    name: &'static str,
    ninputs: i32,
    input_type: Type,
}

pub fn create_arg(name: &'static str) -> Argument {
    let mut optional = false;
    if name.contains("-") {
        optional = true;
    }

    return Argument {
        name,
        aliases: Vec::new(),
        params: Vec::new(),
        optional,
        help: String::new(),
    }
}

// TODO: default values
impl Argument {
    pub fn alias(mut self, name: &'static str) -> Self {
        self.aliases.push(name);
        return self;
    }

    pub fn add_param(mut self, name: &'static str, nargs: i32, input_type: Type) -> Self {
        self.params.push(
            Parameter { 
                name, 
                ninputs: nargs, 
                input_type 
            }
        );

        return self;
    }

    pub fn help(mut self, help_text: &'static str) -> Self {
        self.help = help_text.to_owned();
        return self;
    }

    pub fn add(self, parser: &mut Clip) {
        if !self.optional {
            parser.positional.push(self.name);
        }

        for alias in self.aliases.clone() {
            parser.aliases.insert(alias, self.name);
        }

        parser.args.insert(self.name, self);
    }
}

#[derive(Debug)]
pub struct Input<'a> {
    name: &'static str,
    values: Vec<&'a str>,
}

#[derive(Debug)]
pub struct Clip {
    program_name: &'static str,
    positional: Vec<&'static str>,
    aliases: HashMap<&'static str, &'static str>,
    args: HashMap<&'static str, Argument>,
    env_args: Vec<String>,
}

impl Clip {
    pub fn new(program_name: &'static str) -> Self {
        Self {
            program_name,
            positional: Vec::new(),
            aliases: HashMap::new(),
            args: HashMap::new(),
            env_args: Vec::new(),
        }
    }

    pub fn add(&mut self, arg: Argument) {
        arg.add(self);
    }

    pub fn debug(&self) {
        // TODO:
        dbg!(self);
    }

    pub fn parse_env(&mut self) -> Result<Vec<Input<'_>>, Error> {
        self.env_args = std::env::args().skip(1).collect::<Vec<String>>();
        return self.parse_vec(self.env_args.iter().map(|x| x.as_str()));
    }

    pub fn parse<'a>(&mut self, input: &'a String) -> Result<Vec<Input<'a>>, Error> {
        return self.parse_vec(input.split(" "));
    }
}

impl Clip {
    fn parse_vec<'a>(&self, input: impl Iterator<Item = &'a str>) -> Result<Vec<Input<'a>>, Error> {
        let mut inputs: Vec<Input> = Vec::new();
        let mut iter = input.peekable();
        while let Some(mut arg_input) = iter.next() {
            if let Some(alias) = self.aliases.get(arg_input) {
                arg_input = alias;
            }

            let Some(arg) = self.args.get(arg_input) else {
                return Err(Error::UnknownArgument(arg_input.to_string()));
            };

            inputs.push(self.parse_arg(&mut iter, arg)?);
        };

        return Ok(inputs);
    }

    fn parse_arg<'a>(&self, input: &mut std::iter::Peekable<impl Iterator<Item = &'a str>>, arg: &Argument) -> Result<Input<'a>, Error> {
        let mut values: Vec<&str> = Vec::new();
        for param in &arg.params {
            match param.ninputs {
                -1 => {
                    while let Some(next) = input.peek() {
                        if let Some(_) = self.aliases.get(next) {
                            break;
                        }

                        if let Some(_) = self.args.get(next) {
                            break;
                        }

                        values.push(next);
                        _ = input.next();
                    }

                }
                _ => {
                    for _ in 0..param.ninputs {
                        let Some(input_param) = input.next() else {
                            return Err(Error::ExpectedParameter { 
                                argument: arg.name.to_owned(),
                                parameter: param.name.to_owned() 
                            });
                        };

                        values.push(input_param);
                    }
                }
            }
        }

        return Ok(Input {
            name: arg.name,
            values
        });
    }
}
