use std::vec::Vec;
use std::collections::HashMap;

// TODO: general API todos
// .to_owned(), creates dynamic structures without lifetimes
// error checking:
//      variadic params
//      types 
// error printing
// tests
// commands

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
        if self.params.len() > 0 {
            if self.params[self.params.len() - 1].ninputs == -1 {
                // TODO: better errors
                panic!("A parameter with variadic args must be the last parameter in an argument.");
            }
        }
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

#[derive(Debug, Clone)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard() {
        let mut parser = Clip::new("foo");
        create_arg("--file")
            .alias("-f")
            .add_param("file", 1, Type::String)
            .help("input file")
            .add(&mut parser);

        assert_eq!(parser.program_name, "foo");
        assert_eq!(parser.args.len(), 1);
        assert_eq!(parser.aliases.len(), 1);

        let alias_option = parser.aliases.get("-f");
        assert_eq!(alias_option, Some(&"--file"));

        let arg_option = parser.args.get("--file");
        assert!(arg_option.is_some());
        let arg = arg_option.unwrap();

        assert_eq!(arg.help, "input file");
        assert_eq!(arg.params.len(), 1);

        let param = &arg.params[0];
        assert_eq!(param.name, "file");
        assert_eq!(param.ninputs, 1);
        assert_eq!(param.input_type, Type::String);

        let string = "--file foo.rs".to_owned();
        let inputs = parser.parse(&string);
        assert!(inputs.is_ok());
        let vals = inputs.unwrap();

        assert_eq!(vals.len(), 1);
        assert_eq!(vals[0].name, "--file");
        assert_eq!(vals[0].values.len(), 1);
        assert_eq!(vals[0].values[0], "foo.rs");
    }

    #[test]
    fn aliases() {
        let mut parser = Clip::new("foo");
        create_arg("--file")
            .alias("-f")
            .add_param("file", 1, Type::String)
            .help("input file")
            .add(&mut parser);

        create_arg("--output")
            .alias("-o")
            .alias("--out")
            .add_param("output", 1, Type::String)
            .help("output file")
            .add(&mut parser);

        let input1 = "--file foo.rs -o out.o".to_owned();
        let input2 = "--out out.o -f foo.rs".to_owned();
        let res = parser.parse(&input1);
        let res2 = parser.parse(&input2);

        assert!(res.is_ok());
        assert!(res2.is_ok());

        // vals2 will parse in reverse order but should be the same values
        let vals = res.unwrap().into_iter();
        let vals2 = res2.unwrap().into_iter().rev();

        for (v1, v2) in std::iter::zip(vals, vals2) {
            assert_eq!(v1.name, v2.name);
            assert_eq!(v1.values, v2.values);
        }
    }

    // tests both variadic in the middle of the command and at the end
    #[test]
    fn variadic() {
        let mut parser = Clip::new("foo");
        create_arg("--file")
            .alias("-f")
            .add_param("file", -1, Type::String)
            .help("input file")
            .add(&mut parser);

        create_arg("--output")
            .alias("-o")
            .alias("--out")
            .add_param("output", 1, Type::String)
            .help("output file")
            .add(&mut parser);

        let i1 = &"--file foo.rs bar.rs baz.rs -o out.o".to_owned();
        let i2 = &"-o out.o --file foo.rs bar.rs baz.rs".to_owned();
        let res = parser.parse(i1);
        let res2 = parser.parse(i2);

        assert!(res.is_ok());
        assert!(res2.is_ok());

        let vals = res.unwrap().into_iter();
        let vals2 = res2.unwrap().into_iter().rev();

        for (v1, v2) in std::iter::zip(vals.clone(), vals2) {
            assert_eq!(v1.name, v2.name);
            assert_eq!(v1.values, v2.values);
        }

        let inputs = vals.collect::<Vec<Input<'_>>>();
        assert_eq!(inputs.len(), 2);
        assert_eq!(inputs[0].name, "--file");
        assert_eq!(inputs[0].values.len(), 3);

        assert_eq!(inputs[0].values, vec!["foo.rs", "bar.rs", "baz.rs"]);
    }
}
