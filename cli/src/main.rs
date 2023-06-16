use clap::{App, Arg};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define command-line arguments using clap
    let matches = App::new("Lessy")
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .value_name("INPUT_FILE")
                .help("Sets the input file")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("OUTPUT_FILE")
                .help("Sets the output file")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("replace")
                .short("r")
                .long("replace")
                .value_name("TEXT_TO_REPLACE")
                .help("Sets the text to be replaced")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    // Retrieve values of command-line arguments
    let input_file = matches.value_of("input").unwrap();
    let output_file = matches.value_of("output").unwrap();
    let text_to_be_replaced = matches.value_of("replace").unwrap();

    // Rest of your code
    lessy::run(input_file, output_file, text_to_be_replaced)
}
