use clap::Parser as ArgParser;

mod parser;

/// Provide file
#[derive(ArgParser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// JS file to convert from {} import ... to const {} = await import(...)
    file: String,
}

fn main() -> std::io::Result<()> {
    // read a file to convert the from import to const {} import (`...`)
    let args = Args::parse();

    // let contents = fs::read_to_string(args.file.clone())
    //     .expect("Provided file can not be read");
    //
    // let _ = fs::write(args.file.clone(), parser::parse(contents));

    parser::parse(args.file.clone())?;

    println!("Parsed file '{}'", args.file);

    Ok(())
}
