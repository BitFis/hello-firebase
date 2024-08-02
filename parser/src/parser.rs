use std::fs;
use pest::iterators::Pairs;
use pest::Parser;
use regex::Regex;

#[derive(pest_derive::Parser)]
#[grammar = "import.pest"] // relative to src
struct ImportParser;

pub fn parse(file: String) -> std::io::Result<()> {
    let contents = fs::read_to_string(file.clone())
        .expect("Provided file can not be read");

    fs::write(file, parse_content(contents))?;
    Ok(())
}

fn convert_import(parse: Pairs<'_, Rule>) -> String {
    let mut output = String::from("const {");

    for pair in parse {
        match pair.as_rule() {
            Rule::variable => {

                let inner: Vec<_> = pair.into_inner().collect();

                let impvar = inner[0].as_str();
                let mut defvar = inner[0].as_str();
                if inner.len() > 1 {
                    defvar = inner[1].as_str();
                }

                output.push_str(defvar);
                output.push_str(": ");
                output.push_str(impvar);
                output.push(',');
            }
            Rule::from => {
                output.push_str("} = await import");
            }
            Rule::file_name => {
                output.push_str("(`");
                output.push_str(pair.as_str());
                output.push_str("${importquery}`)");
            }
            _ => {
                println!("unprocessed {}", pair.as_str())
            }
        }
    }

    output.push_str(";");

    return output;
}

fn parse_content(content: String) -> String {
    let parse = ImportParser::parse(Rule::main, &content)
        .unwrap_or_else(|e| panic!("{}", e))
        .next()
        .unwrap();

    let mut output = String::from("");

    for pair in parse.into_inner() {
        match pair.as_rule() {
            Rule::import_block => {
                output.push_str(&convert_import(pair.into_inner()));
            }
            _ => {
                output.push_str(pair.as_str());
            }
        }
    }

    return output;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn trim(content: String) -> String {
        let re = Regex::new(r"(  +|\n)").unwrap();
        re.replace_all(&content, "").to_string()
    }

    #[test]
    fn parse_simple_content_replace() {
        let input = String::from("
        import {
            __async
        } from \"./chunk-H3KQGMCP.js\";
        var well = true;
        ");
        let expect = String::from("
        const {
            __async: __async,
        } = await import(`./chunk-H3KQGMCP.js${importquery}`);
        var well = true;");
        assert_eq!(trim(parse_content(input)), trim(expect));
    }

    #[test]
    fn parse_import_as_rename() {
        let input = String::from("
        import{a as It,b as wt}from'./chunk-2RG4V45Z.js';var sr=null;var ir=1,fi=Symbol(\"SIGNAL\");function
        ");
        let expect = String::from("
        const {
            It: a,
            wt: b,
        } = await import(`./chunk-2RG4V45Z.js${importquery}`);
        var sr=null;var ir=1,fi=Symbol(\"SIGNAL\");function");
        assert_eq!(trim(parse_content(input)), trim(expect));
    }

    #[test]
    fn parse_test() -> std::io::Result<()> {
        fs::copy("tests/debug-import.js", "tests/debug-import-actual.js")?;
        fs::remove_file("tests/debug-import-actual.js")?;
        Ok(())
    }
}
