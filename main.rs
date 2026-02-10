fn main() {
    let mut cli_parser = clip::Clip::new("Clip");
    clip::create_arg("--file")
    .alias("-f")
    .input("files", -1, clip::Type::String)
    .help("specifies the input files")
    .add(&mut cli);

    cli_parser.parse("-f foo.rs bar.rs");

    // or
    // cli_parser.add(
    //     clip::create_arg("--file")
    //     .alias("-f")
    //     .input("files", -1, clip::Type::String)
    //     .help("specifies the input files")
    // );

    cli_parser.debug();
}
