use clip::{ Clip, Type, create_arg };

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut parser = Clip::new("Clip");
    create_arg("input")
        .positional(Type::File)
        .help("a file to modify")
        .add(&mut parser);
    
    create_arg("pos")
        .positional(Type::Integer)
        .help("a position to insert at")
        .add(&mut parser);
}

// fn main() {
//     let mut parser = clip::Clip::new("Clip");
//     clip::create_arg("input")
//         .positional(clip::Type::File)
//         .help("input")
//         .add(&mut parser);
//
//     clip::create_arg("mode")
//         .positional(clip::Type::Set(&["read", "write", "append"]))
//         .help("file modes")
//         .add(&mut parser);
//
//     clip::create_arg("output")
//         .positional(clip::Type::File)
//         .help("output")
//         .add(&mut parser);
//
//     let input: String = String::from("data.txt read out.txt");
//     let res = parser.parse(&input).unwrap_or_else(|e| {
//         eprintln!("{}", e);
//         std::process::exit(1);
//     });
//
//     let [input, mode, output] = res.to_owned().as_slice() else {
//         unreachable!();
//     };
// }
