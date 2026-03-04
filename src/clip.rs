use std::collections::HashMap;
use crate::arg::Argument;
use crate::arg::Mode;
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
    pub(crate) variadic: Option<&'static str>,
    pub(crate) aliases: HashMap<&'static str, &'static str>,
    pub(crate) args: HashMap<&'static str, Argument>,
}

impl Clip {
    pub fn new(program_name: &'static str) -> Self {
        Self {
            program_name,
            positional: Vec::new(),
            variadic: None,
            aliases: HashMap::new(),
            args: HashMap::new(),
        }
    }

    pub fn add(&mut self, arg: Argument) {
        arg.add(self);
    }

    pub fn parse<'a>(&mut self, input: &'a String) -> Result<Vec<Input<'a>>, Error> {
        return self.parse_vec(input.split(" "));
    }
}

impl Clip {
    fn parse_vec<'a>(
        &self, 
        input: impl Iterator<Item = &'a str>
    ) -> Result<Vec<Input<'a>>, Error> {
        let mut inputs: Vec<Input> = Vec::new();
        let mut iter = input.peekable();

        if let Some(variadic) = self.variadic {
            inputs.push( Input{
                name: variadic,
                values: Vec::new(),
            })
        }

        while let Some(mut arg_input) = iter.next() {
            if let Some(alias) = self.aliases.get(arg_input) {
                arg_input = alias;
            }

            if let Some(arg) = self.args.get(arg_input) {
                inputs.push(self.parse_arg(&mut iter, arg)?);
                continue;
            };

            if let Some(_) = self.variadic {
                // TODO: calculate index off of n positional args
                inputs[0].values.push(arg_input);
                continue;
            }

            return Err(Error::UnknownArgument(arg_input.to_string()));
        };

        return Ok(inputs);
    }

    fn parse_arg<'a>( 
        &self, 
        input: &mut std::iter::Peekable<impl Iterator<Item = &'a str>>, 
        arg: &Argument
    ) -> Result<Input<'a>, Error> {
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
                            return Err( Error::ExpectedParameter( param.name.to_owned()));
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
