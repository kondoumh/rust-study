use clap::{crate_authors, crate_version, App, Arg};



fn main() {
    let matches = App::new("grep")
        .version(crate_version!())
        .author(crate_authors!())
        .arg(
            Arg::with_name("fixed-strings")
                .short("F")
                .long("fixed-strings")
                .help("PATTERNS ar strings"),
        )
        .arg(
            Arg::with_name("PATTERNS")
                .help("use PATTERNS for matching")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("FILES")
                .help("take PATTERNS from FILES")
                .required(true)
                .multiple(true)
                .index(2)
        )
        .get_matches();
    let pattern = matches.value_of("PATTERNS").unwrap();
    let file_pathes = matches
        .values_of("FILES")
        .unwrap()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    let is_fixed_string_mode = matches.is_present("fixed-strings");
    println!("{:?}", matches)
}
