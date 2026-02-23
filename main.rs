fn main() {
    let mut parser = clip::Clip::new("Clip");
    clip::create_arg("--file")
        .alias("-f")
        .add_param("file", -1, clip::Type::File)
        .help("specifies the input file")
        .add(&mut parser);

    clip::create_arg("--output")
        .alias("-o")
        .add_param("file", 1, clip::Type::File)
        .help("specifies the input file")
        .add(&mut parser);

    dbg!(parser.parse(&"-f foo.rs bar.rs -o out.o".to_string()));
    dbg!(parser.parse_env());

    // or
    // cli_parser.add(
    //     clip::create_arg("--file")
    //     .alias("-f")
    //     .input("files", -1, clip::Type::String)
    //     .help("specifies the input files")
    // );

    // cli_parser.debug();
}
