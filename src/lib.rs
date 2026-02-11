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
        parameters: String,
    }
}

#[derive(Clone, Debug)]
pub struct ParserArg {
    name: String,
    aliases: Vec<&'static str>,
    params: Vec<Parameter>,
    optional: bool,
    help: String,
}

#[derive(Clone, Debug, PartialEq)]
struct Parameter {
    name: &'static str,
    ninputs: i32,
    input_type: Type,
}

pub fn create_arg(name: &'static str) -> ParserArg {
    let formatted_name = name.replace("-", "");
    let mut optional = false;
    if name.contains("-") {
        optional = true;
    }

    return ParserArg {
        name: formatted_name,
        aliases: Vec::new(),
        params: Vec::new(),
        optional,
        help: String::new(),
    }
}

// TODO: default values
impl ParserArg {
    pub fn alias(&mut self, name: &'static str) -> &mut Self {
        self.aliases.push(name);
        return self;
    }

    pub fn add_param(&mut self, name: &'static str, nargs: i32, input_type: Type) -> &mut Self {
        self.params.push(
            Parameter { 
                name, 
                ninputs: nargs, 
                input_type 
            }
        );

        return self;
    }

    pub fn help(&mut self, help_text: &'static str) -> &mut Self {
        self.help = help_text.to_owned();
        return self;
    }

    pub fn add(&self, parser: &mut Clip) {
        parser.args.insert(self.name.clone(), self.clone());
        if !self.optional {
            parser.positional.push(self.name.clone());
        }

        for alias in self.aliases.clone() {
            parser.aliases.insert(alias.to_owned(), self.name.clone());
        }
    }
}

#[derive(Debug)]
pub struct Argument {
    name: String,
    values: Vec<String>,
}

#[derive(Debug)]
enum State {
    GetArg,
    GetParam,
    GetInput,
    // BuildArg,
}

#[derive(Debug)]
pub struct Clip {
    program_name: &'static str,
    positional: Vec<String>,
    aliases: HashMap<String, String>,
    args: HashMap<String, ParserArg>,

    // State machine
    state: State,
    curr_arg: Option<ParserArg>,
    curr_param: Option<Parameter>,
    nparam: usize,
    ninput: usize,
    variadic: bool,
    vals: Vec<String>,
}

impl Clip {
    pub fn new(program_name: &'static str) -> Self {
        Self {
            program_name,
            positional: Vec::new(),
            aliases: HashMap::new(),
            args: HashMap::new(),

            state: State::GetArg,
            curr_arg: None,
            curr_param: None,
            nparam: 0,
            ninput: 0,
            variadic: false,
            vals: Vec::new(),
        }
    }

    pub fn add(&mut self, arg: &mut ParserArg) {
        arg.add(self);
    }

    pub fn debug(&self) {
        // TODO:
        dbg!(self);
    }

    pub fn parse_env(&mut self) -> Result<Vec<Argument>, Error> {
        return self.parse_vec(
            std::env::args()
            .skip(1) // skips over ./main
            .collect::<Vec<String>>()
        );
    }

    pub fn parse(&mut self, input: String) -> Result<Vec<Argument>, Error> {
        return self.parse_vec(
            input.split(" ")
            .map(str::to_string)
            .collect::<Vec<String>>()
        );
    } 
}

impl Clip {
    fn get_arg(&mut self, word: String) -> Result<Option<Argument>, Error> {
        let lookup = match self.aliases.get(&word) {
            Some(original) => original.clone(),
            None => word,
        };

        let Some(v) = self.args.get(&lookup.replace("-", "")) else {
            return Err(Error::UnknownArgument(lookup));
        };

        self.state = State::GetParam;
        self.curr_arg = Some(v.clone());
        return Ok(None);
    }

    fn get_param(&mut self, word: String) -> Result<Option<Argument>, Error> {
        let arg = self.curr_arg.as_mut().expect("Clip internal error: invalid state: parsing param without arg");

        if self.nparam >= arg.params.len() {
            return Ok(Some(self.build_arg()));
        }

        self.curr_param = Some(arg.params[self.nparam].clone());
        self.nparam += 1;
        self.state = State::GetInput;
        return self.get_input(word); // i have to add this call as each word only gets one state,
                                     // but this state doesnt consume the word
    }

    fn get_input(&mut self, word: String) -> Result<Option<Argument>, Error> {
        let param = self.curr_param.as_mut().expect("Clip internal error: invalid state: parsing input without param");
        // this conversion should be okay, as ninputs > 0 as long as its not variadic
        if !self.variadic && self.ninput >= param.ninputs as usize {
            self.state = State::GetParam;
            return self.get_param(word); // same reason as earlier
        }

        self.vals.push(word);
        return Ok(None);
    }

    fn build_arg(&mut self) -> Argument {
        let Some(arg) = self.curr_arg.clone() else {
            panic!("Clip internal error: can not call Clip::build_arg() without a defined arg");
        };

        let name = arg.name;
        let values = self.vals.clone();

        self.state = State::GetArg;
        self.curr_arg = None;
        self.curr_param = None;
        self.nparam = 0;
        self.ninput = 0;
        self.variadic = false;
        self.vals = Vec::new();

        Argument { name, values }
    }

    fn parse_word(&mut self, word: String) -> Result<Option<Argument>, Error> {
        match self.state {
            State::GetArg => self.get_arg(word),
            State::GetParam => self.get_param(word),
            State::GetInput => self.get_input(word),
            // State::BuildArg => self.build_arg(),
        }
    }

    fn parse_vec(&mut self, input: Vec<String>) -> Result<Vec<Argument>, Error> {
        let mut args: Vec<Argument> = Vec::new();
        for word in input {
            match self.parse_word(word) {
                Ok(arg) => {
                    if let Some(a) = arg {
                        args.push(a);
                    }
                }
                Err(e) => return Err(e),
            }
        }

        return Ok(args);
    }
}

// struct StateMachine {
//     state: State,
//     arg: Option<ParserArg>,
//     param: Option<Parameter>,
//     ninput: usize,
//     variadic: bool,
// }
//
// impl StateMachine {
//     pub fn new() -> Self {
//         Self {
//             state: State::GetArg,
//             arg: None,
//             param: None,
//             ninput: 0,
//             variadic: false,
//         }
//     } 
//
//     fn parse(&self, word: String) -> Option<Error> {
//         match self.state {
//             State::GetArg => self.get_arg(word),
//             State::GetParam => self.get_param(word),
//             State::GetInput => self.get_input(word),
//         }
//     }
//
//     fn get_arg(&self, word: String) -> Option<Error> {
//         let lookup = match self.aliases.get(&word) {
//             Some(original) => original.clone(),
//             None => word,
//         };
//
//         let Some(v) = self.args.get(&lookup) else {
//             return Err(Error::UnknownArgument(lookup));
//         };
//
//         curr_arg = Some(v.clone());
//     }
//
//     fn get_param(&self, word: String) -> Option<Error> {
//
//     }
//
//     fn get_input(&self, word: String) -> Option<Error> {
//
//     }
// }

// // actual parsing
// impl Clip {
//     fn parse_vec(&self, input: Vec<String>) -> Result<Vec<Argument>, Error> {
//         let state_machine = StateMachine::new();
//         let arguments: Vec<Argument> = Vec::new();
//
//         for word in input {
//             // get the next arg
//             let Some(arg) = curr_arg.clone() else {
//                 let lookup = match self.aliases.get(&word) {
//                     Some(original) => original.clone(),
//                     None => word,
//                 };
//
//                 let Some(v) = self.args.get(&lookup) else {
//                     return Err(Error::UnknownArgument(lookup));
//                 };
//
//                 curr_arg = Some(v.clone());
//                 continue;
//             };
//
//             // get the inputs
//             let Some(arg_input) = curr_input.clone() else {
//                 arg.inputs = 
//             };
//
//
//         }
//
//         return Ok(arguments);
//     }
// }

// actual parsing
// impl Clip {
//     fn current(&self) -> Option<String> {
//         if self.i >= self.input.len() {
//             return Some(self.input[self.i].clone());
//         }
//
//         return None;
//     }
//
//     fn advance(&mut self) {
//         self.i += 1;
//     }
//
//     fn consume(&mut self) -> Option<String> {
//         let out = self.current();
//         self.advance();
//         return out;
//     }
//
//     fn parse_arg(&mut self, arg: ParserArg) -> Argument {
//         for input in parser_arg.inputs.clone() {
//             for _ in 0..input.nargs {
//                 let Some(next) = self.consume() else {
//                     return Err(Error::ExpectedInput {
//                         argument: parser_arg.name.clone(),
//                         input_name: input.name.to_string(),
//                     });
//                 };
//             }
//         }
//
//         return Argument {
//             name: String::new(),
//             val: Vec::new(),
//         }
//     }
//
//     fn parse_vec(&mut self, input: Vec<String>) -> Result<Vec<Argument>, Error> {
//         self.input = input;
//         let args: Vec<Argument> = Vec::new();
//
//         while let Some(word) = self.current() {
//             let arg = match self.aliases.get(&word) {
//                 Some(original) => original.clone(),
//                 None => word,
//             };
//
//             let Some(parser_arg) = self.args.get(&arg) else {
//                 return Err(Error::UnknownArgument(arg));
//             };
//
//         }
//
//         return Ok(args);
//     }
// }

// actual parsing
// impl Clip {
    // fn parse_vec(&self, input: Vec<String>) -> Result<Vec<Argument>, Error> {
    //     let mut i: usize = 0;
    //     let mut args: Vec<Argument> = Vec::new();
    //     while i < input.len() {
    //         let curr = &input[i];
    //
    //         let Some(parser_arg) = self.args.get(curr) else {
    //             return Err(Error::UnknownArgument(curr.clone()));
    //         };
    //
    //         for arg_input in parser_arg.inputs.clone() {
    //             let mut variadic: bool = false;
    //             let mut ninput: usize = 0;
    //             if arg_input.nargs < 0 {
    //                 variadic = true;
    //             }
    //
    //             loop {
    //                 if !variadic && ninput >= arg_input.nargs as usize {
    //                     break;
    //                 }
    //
    //                 i += 1;
    //                 if i >= input.len() {
    //                     return Err(Error::ExpectedInput {
    //                         argument: parser_arg.name.clone(),
    //                         input_name: arg_input.name.to_string(),
    //                     });
    //                 }
    //             }
    //         }
    //     }
    //
    //     return Ok(args);
    // }
// }

// actual parsing
// impl Clip {
//     fn current(&self) -> Option<String> {
//         if self.i >= self.input.len() {
//             return Some(self.input[self.i].clone());
//         }
//
//         return None;
//     }
//
//     fn advance(&mut self) {
//         self.i += 1;
//     }
//
//     fn consume(&mut self) -> Option<String> {
//         let out = self.current();
//         self.advance();
//         return out;
//     }
//
//     fn parse_vec(&self, input: Vec<String>) -> Result<Vec<Argument>, Error> {
//         // self.input = input;
//         let args: Vec<Argument> = Vec::new();
//
//         while let Some(word) = self.current() {
//             let arg = match self.aliases.get(&word) {
//                 Some(original) => original.clone(),
//                 None => word,
//             };
//
//             let Some(parser_arg) = self.args.get(&arg) else {
//                 return Err(Error::UnknownArgument(arg));
//             };
//
//             for input in parser_arg.inputs.clone() {
//                 for _ in 0..input.nargs {
//                     let Some(next) = self.consume() else {
//                         return Err(Error::ExpectedInput {
//                             argument: parser_arg.name,
//                             input_name: input.name.to_string(),
//                         });
//                     };
//                 }
//             }
//         }
//
//         return Ok(args);
//
//         // for word in input {
//         //     let arg = match self.aliases.get(&word) {
//         //         Some(original) => original.clone(),
//         //         None => word,
//         //     };
//         //
//         //     let Some(parser_arg) = self.args.get(&arg) else {
//         //         return Err(Error::UnknownArgument(arg));
//         //     };
//         //
//         //     for input in parser_arg.inputs.clone() {
//         //         dbg!(input);
//         //     }
//         // }
//         //
//         // return Ok(args);
//     }
// }
