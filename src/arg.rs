use crate::clip::Clip;

#[derive(Debug, PartialEq)]
pub enum Type {
    Any,
    Integer,
    Number,
    String,
    File,
    Set(&'static [&'static str]),
    Range {
        lower: i32,
        upper: i32,
    },
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Type::Any => write!(f, "Any"),
            Type::Integer => write!(f, "Integer"),
            Type::Number => write!(f, "Number"),
            Type::String => write!(f, "String"),
            Type::File => write!(f, "File"),
            Type::Set(vals) => {
                let set = vals.join(", ");
                return write!(f, "Set::[{}]", set);
            },
            Type::Range {
                lower,
                upper
            } => write!(f, "Range::[{}-{}]", lower, upper),
        }
    }
}

#[derive(Debug)]
pub(crate) enum Mode {
    Flag,
    Positional,
    Variadic,
}

#[derive(Debug)]
pub struct Argument {
    pub(crate) name: &'static str,
    pub(crate) aliases: Vec<&'static str>,
    pub(crate) params: Vec<Parameter>,
    pub(crate) help: String,
    pub(crate) arg_type: Type,
    pub(crate) mode: Mode,
}

#[derive(Debug, PartialEq)]
pub(crate) struct Parameter {
    pub(crate) name: &'static str,
    pub(crate) ninputs: i32,
    pub(crate) input_type: Type,
}

impl Argument {
    pub fn positional(mut self, t: Type) -> Self {
        self.mode = Mode::Positional;
        self.arg_type = t;
        return self;
    }

    pub fn variadic(mut self, t: Type) -> Self {
        self.mode = Mode::Variadic;
        self.arg_type = t;
        return self;
    }

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
        self.aliases.iter().for_each(|alias| {
            parser.aliases.insert(alias, self.name);
        });

        parser.args.insert(self.name, self);
    }
}

pub fn create_arg(name: &'static str) -> Argument {
    return Argument {
        name,
        aliases: Vec::new(),
        params: Vec::new(),
        help: String::new(),
        arg_type: Type::Any,
        mode: Mode::Flag,
    }
}
