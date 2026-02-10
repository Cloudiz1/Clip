use std::vec::Vec;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum Type {
    String,
    Integer,
    // Number,
}

#[derive(Clone, Debug)]
pub struct ClipArgument {
    name: String,
    aliases: Vec<&'static str>,
    inputs: Vec<ArgumentInput>,
    optional: bool,
    help: String,
}

#[derive(Clone, Debug)]
struct ArgumentInput {
    name: &'static str,
    nargs: i32,
    input_type: Type,
}

pub fn create_arg(name: &'static str) -> ClipArgument {
    let formatted_name = name.replace("-", "");
    let mut optional = false;
    if name.contains("-") {
        optional = true;
    }

    return ClipArgument {
        name: formatted_name,
        aliases: Vec::new(),
        inputs: Vec::new(),
        optional,
        help: String::new(),
    }
}

// struct ArgumentInput {
//     name: &'static str,
//     nargs: i32,
//     input_type: Type,
// }

impl ClipArgument {
    pub fn alias(&mut self, name: &'static str) -> &mut Self {
        self.aliases.push(name);
        return self;
    }

    pub fn input(&mut self, name: &'static str, nargs: i32, input_type: Type) -> &mut Self {
        self.inputs.push(
            ArgumentInput { 
                name, 
                nargs, 
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
        parser.args.insert(self.name.to_owned(), self.clone());

        // TODO: it might be worth it to store aliases differently
        // this is certainly not very memory efficient
        for alias in self.aliases.clone() {
            parser.args.insert(alias.to_owned(), self.clone());
        }
    }
}

// pub struct ClipArgument {
//     inputs: Vec<ArgumentInput>,
//     help: &'static str,
// }

#[derive(Debug)]
pub struct Clip {
    program_name: &'static str,
    i: usize,
    args: HashMap<String, ClipArgument>,
}

impl Clip {
    pub fn new(program_name: &'static str) -> Self {
        Self {
            program_name,
            i: 0,
            args: HashMap::new(),
        }
    }

    pub fn parse(&self, input: String) -> Vec<ClipArgument> {
        Vec::new()
    } 

    pub fn add(&mut self, arg: &mut ClipArgument) {
        arg.add(self);
    }

    pub fn debug(&self) {
        dbg!(self);
    }
}
