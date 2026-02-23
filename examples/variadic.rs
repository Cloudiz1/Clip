fn main() {
    let mut parser = clip::Clip::new("Clip");
    clip::create_arg("input files")
        .variadic(clip::Type::File)
        .help("input files")
        .add(&mut parser);

    clip::create_arg("--output")
        .alias("-o")
        .add_param("output", 1, clip::Type::File)
        .help("output file")
        .add(&mut parser);

    let input: String = String::from("input1 input2 --output a.out input3");
    let res = parser.parse(&input).unwrap_or_else(|e| {
        eprintln!("{}", e);
        std::process::exit(1);
    });
}
