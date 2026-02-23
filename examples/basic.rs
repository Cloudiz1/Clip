fn main() {
    let mut parser = clip::Clip::new("Clip");
    clip::create_arg("--output")
        .alias("-o")
        .add_param("output", 1, clip::Type::File)
        .help("output file")
        .add(&mut parser);

    /* or
     * cli_parser.add(
     *      clip::create_arg("--output")
     *      .alias("-o")
     *      .input("output", 1, clip::Type::File)
     *      .help("output file")
     * );
     */

    clip::create_arg("--file")
        .alias("-f")
        .add_param("file", -1, clip::Type::File)
        .help("input files")
        .add(&mut parser);

    let input: String = String::from("--output a.out -f main.rs util.rs");
    let res = parser.parse(&input).unwrap_or_else(|e| {
        eprintln!("{}", e);
        std::process::exit(1);
    });

    /* or
     * parser.parse_env()
     * parser.parse(&input);
     */
}
