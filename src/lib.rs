// general API todos
// .to_owned(), creates dynamic structures without lifetimes
// error checking:
//      variadic params
//      types 
// more types!
//      file type
//      general number type
//      maybe even a custom type that restricts inputs to a limited set
// error printing
// tests
// commands
// a debug command to pretty print information about the CLI parser
// help command
// defaults

mod arg;
mod clip;
mod error;

pub use crate::clip::Clip;
pub use crate::arg::create_arg;
pub use crate::arg::Type;
pub use crate::clip::Input;
pub use crate::error::Error;

#[cfg(test)]
mod tests {
    use crate::clip;
    use crate::arg::create_arg;
    use crate::arg::Type;
    use crate::clip::Input;

    #[test]
    fn standard() {
        let mut parser = clip::Clip::new("foo");
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
        let mut parser = clip::Clip::new("foo");
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
        let mut parser = clip::Clip::new("foo");
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
