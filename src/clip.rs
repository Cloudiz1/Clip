use std::collections::HashMap;
use crate::arg::Argument;
use crate::error::Error;

#[derive(Debug, Clone)]
pub struct Input<'a> {
    pub name: &'static str,
    pub values: Vec<&'a str>,
}

#[derive(Debug)]
pub struct Clip {
    pub(crate) program_name: &'static str,
    pub(crate) positional: Vec<&'static str>,
    pub(crate) aliases: HashMap<&'static str, &'static str>,
    pub(crate) args: HashMap<&'static str, Argument>,
    pub(crate) env_args: Vec<String>,
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
