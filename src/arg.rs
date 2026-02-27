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

#[derive(Debug, PartialEq)]
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
        #[cfg(debug_assertions)]
        if self.params.len() > 0 {
            if self.params[self.params.len() - 1].ninputs == -1 {
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

    #[cfg(debug_assertions)]
    fn verify(&self, parser: &Clip) {
        if self.mode != Mode::Flag {
            if self.aliases.len() != 0 {
                panic!("Argument {} can not have an alias as it is of mode {:#?}.\n
                    Only Mode::Flag has a name, and therefore only Mode::Flag can have an alias", 
                    self.name, self.mode);
            }

            if self.params.len() != 0 {
                panic!("Argument {} can not have parameters as it is of mode {:#?}", self.name, self.mode);
            }
        }

        if parser.args.contains_key(self.name) {
            panic!("Argument {} already exists", self.name);
        }

        for alias in &self.aliases {
            if parser.aliases.contains_key(alias) {
                panic!("Alias {} already exists", alias);
            }
        }

        let mut variadic: Vec<&Argument> = Vec::new();
        parser.args.values().for_each(|arg| {
            if let Mode::Variadic = arg.mode {
                variadic.push(arg);
            }
        });

        if variadic.len() > 1 {
            panic!("Expected 0 or 1 variadic arguments, found {}", variadic.len());
        }
    }

    pub fn add(self, parser: &mut Clip) {
        #[cfg(debug_assertions)]
        self.verify(parser);

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

#[cfg(test)]
mod tests {
    use crate::Clip;
    use crate::Type;
    use crate::create_arg;

    #[test]
    #[should_panic]
    fn variadic_fail() {
        let mut parser = Clip::new("foo");
        create_arg("--file")
            .alias("-f")
            .add_param("file", -1, Type::String)
            .add_param("config_file", 1, Type::String)
            .help("input file")
            .add(&mut parser);
    }

    #[test]
    #[should_panic]
    fn erreneous_alias() {
        let mut parser = Clip::new("foo");
        create_arg("file")
            .positional(Type::File)
            .alias("-f")
            .help("input file")
            .add(&mut parser);
    }

    #[test]
    #[should_panic]
    fn erreneous_param() {
        let mut parser = Clip::new("foo");
        create_arg("file")
            .variadic(Type::File)
            .add_param("foo", 1, Type::String)
            .help("input file")
            .add(&mut parser);
    }

    #[test]
    #[should_panic]
    fn duplicate_args() {
        let mut parser = Clip::new("foo");
        create_arg("file")
            .add_param("foo", 1, Type::String)
            .help("input file")
            .add(&mut parser);

        create_arg("file")
            .add_param("foo", 1, Type::String)
            .help("input file")
            .add(&mut parser);
    }

    #[test]
    #[should_panic]
    fn duplicate_alias() {
        let mut parser = Clip::new("foo");
        create_arg("file")
            .alias("-f")
            .add_param("foo", 1, Type::String)
            .help("input file")
            .add(&mut parser);

        create_arg("foo")
            .alias("-f")
            .add_param("foo", 1, Type::String)
            .help("input file")
            .add(&mut parser);
    }

    #[test]
    #[should_panic]
    fn double_varidiac() {
        let mut parser = Clip::new("foo");
        create_arg("file")
            .variadic(Type::File)
            .alias("-f")
            .add_param("foo", 1, Type::String)
            .help("input file")
            .add(&mut parser);

        create_arg("foo")
            .variadic(Type::Any)
            .alias("-f")
            .add_param("foo", 1, Type::String)
            .help("input file")
            .add(&mut parser);
    }
}
