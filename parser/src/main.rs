use clap::Parser as ArgParser;

mod import_parser;
mod index_parser;

/// Provide file
#[derive(ArgParser, Debug)]
#[command(version, about, long_about = None)]
enum ParserCli {
    /// JS file to convert `from {} import ...` to `const {} = await import(...)`.
    Js(JsArgs),
    /// Convert angular index file to load script/css/... with expanded query (importquery).
    /// Sets `modulepreload` `modules` and `style`.
    Index(IndexArgs),
}

#[derive(clap::Args, Debug)]
#[command(version, about, long_about = None)]
struct JsArgs {
    file: String,
}

#[derive(clap::Args, Debug)]
#[command(version, about, long_about = None)]
struct IndexArgs {
    /// html file to be processed
    file: String,
    /// provide js script files to be injected into the html
    #[arg(short = 'S', value_name = "script", value_hint = clap::ValueHint::DirPath)]
    scripts: Vec<String>,
}

fn main() -> std::io::Result<()> {
    match ParserCli::parse() {
        ParserCli::Js(args) => {
            import_parser::parse(args.file.clone())?;
            println!("Parsed js file '{}'", args.file);
        }
        ParserCli::Index(args) => {
            if args.scripts.len() > 0 {
                println!("Inject scripts: {}", args.scripts.join(","));
            }
            index_parser::parse(args.file.clone(), args.scripts)?;
            println!("Parsed index file '{}'", args.file);
        }
    }
    Ok(())
}
