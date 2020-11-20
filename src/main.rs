use log::trace;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    trace!("starting program");

    let args = Cli::from_args();

    let file = &args.path;

    let result = std::fs::read_to_string(&file);

    let content = match result {
        Ok(content) => content,
        Err(error) => {
            panic!("ruh roh.... \n {}", error);
        }
    };

    grrs::find_matches(&content, &args.pattern, &mut std::io::stdout());
}
