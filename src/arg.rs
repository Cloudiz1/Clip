use crate::clip::Clip;

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    Integer,
    Number,
    String,
    File,
    Set(&'static [&'static str]),
    Any,
}

#[derive(Clone, Debug)]
pub struct Argument {
    pub(crate) name: &'static str,
    pub(crate) aliases: Vec<&'static str>,
    pub(crate) params: Vec<Parameter>,
    pub(crate) optional: bool,
    pub(crate) help: String,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Parameter {
    pub(crate) name: &'static str,
    pub(crate) ninputs: i32,
    pub(crate) input_type: Type,
}

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
