fn main() {
    let mut cli_parser = clip::Clip::new("Clip");
    clip::create_arg("--file")
        .alias("-f")
        .add_param("file", 1, clip::Type::String)
        .help("specifies the input file")
        .add(&mut cli_parser);

    dbg!(cli_parser.parse_env());
    cli_parser.parse("-f foo.rs".to_string());

    // or
    // cli_parser.add(
    //     clip::create_arg("--file")
    //     .alias("-f")
    //     .input("files", -1, clip::Type::String)
    //     .help("specifies the input files")
    // );

    // cli_parser.debug();
}
