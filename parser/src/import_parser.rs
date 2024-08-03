use pest::iterators::{Pair, Pairs};
use pest::Parser;
use std::fs;

#[derive(pest_derive::Parser)]
#[grammar = "import.pest"]
struct ImportParser;

pub fn parse(file: String) -> std::io::Result<()> {
    let contents = fs::read_to_string(file.clone()).expect("Provided file can not be read");

    fs::write(file, parse_content(contents))?;
    Ok(())
}

fn get_file(pair: Pair<Rule>) -> String {
    let mut output = String::from("(`");
    output.push_str(pair.as_str());
    output.push_str("${importquery}`)");
    return output;
}

fn convert_import(parse: Pairs<'_, Rule>) -> String {
    let mut output = String::from("await import");

    for pair in parse {
        match pair.as_rule() {
            Rule::start_variable => {
                output = String::from("const {");
            }
            Rule::variable => {
                let inner: Vec<_> = pair.into_inner().collect();

                let impvar = inner[0].as_str();
                let mut defvar = inner[0].as_str();
                if inner.len() > 1 {
                    defvar = inner[1].as_str();
                }

                output.push_str(impvar);
                output.push_str(": ");
                output.push_str(defvar);
                output.push(',');
            }
            Rule::end_variable => {
                output.push_str("} = ");
            }
            Rule::from => {
                output.push_str("await import");
            }
            Rule::file_name => {
                output.push_str(&get_file(pair));
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
            Rule::inline_import => {
                for inner_pair in pair.into_inner() {
                    match inner_pair.as_rule() {
                        Rule::file_name => {
                            output.push_str("import");
                            output.push_str(&get_file(inner_pair));
                        }
                        _ => {}
                    }
                }
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
    use regex::Regex;

    fn trim(content: String) -> String {
        let re = Regex::new(r"(  +|\n)").unwrap();
        re.replace_all(&content, "").to_string()
    }

    #[test]
    fn parse_simple_content_replace() {
        let input = String::from(
            "
        import {
            __async
        } from \"./chunk-H3KQGMCP.js\";
        var well = true;
        ",
        );
        let expect = String::from(
            "
        const {
            __async: __async,
        } = await import(`./chunk-H3KQGMCP.js${importquery}`);
        var well = true;",
        );
        assert_eq!(trim(parse_content(input)), trim(expect));
    }

    #[test]
    fn parse_import_as_rename() {
        let input = String::from("
        import{a as It,b as wt}from'./chunk-2RG4V45Z.js';var sr=null;var ir=1,fi=Symbol(\"SIGNAL\");function
        ");
        let expect = String::from(
            "
        const {
            a: It,
            b: wt,
        } = await import(`./chunk-2RG4V45Z.js${importquery}`);
        var sr=null;var ir=1,fi=Symbol(\"SIGNAL\");function",
        );
        assert_eq!(trim(parse_content(input)), trim(expect));
    }

    #[test]
    fn parser_import_special_characters() {
        let input = String::from(
            "
        import {ɵɵelementEnd} from \"./chunk-H3KQGMCP.js\";
        ",
        );
        let expect = String::from(
            "
        const {
            ɵɵelementEnd: ɵɵelementEnd,
        } = await import(`./chunk-H3KQGMCP.js${importquery}`);
        ",
        );
        assert_eq!(trim(parse_content(input)), trim(expect));
    }

    #[test]
    fn parser_import_only() {
        let input = String::from(
            "
        import \"./chunk-H3KQGMCP.js\";
        ",
        );
        let expect = String::from(
            "
        await import(`./chunk-H3KQGMCP.js${importquery}`);
        ",
        );
        assert_eq!(trim(parse_content(input)), trim(expect));
    }

    #[test]
    fn parser_import_inline() {
        let input = String::from(
            "
        await import (\"./chunk-H3KQGMCP.js\");
        ",
        );
        let expect = String::from(
            "
        await import(`./chunk-H3KQGMCP.js${importquery}`);
        ",
        );
        assert_eq!(trim(parse_content(input)), trim(expect));
    }

    #[test]
    fn parser_import_inline_yield() {
        let input = String::from(
            "
    return (yield import(\"./chunk-Q6UJZ2PE.js\")).routes;
            ",
        );
        let expect = String::from(
            "
    return (yield import(`./chunk-Q6UJZ2PE.js${importquery}`)).routes;
        ",
        );
        assert_eq!(trim(parse_content(input)), trim(expect));
    }

    #[test]
    fn parse_ignore_string_import() {
        let input = String::from(
            "
        sage += `Alternatively,
        compiler with
        'import(\"@angular/compiler\");' before boo',
        `
        ",
        );
        assert_eq!(trim(parse_content(input.clone())), trim(input));
    }

    #[test]
    fn parse_test() -> std::io::Result<()> {
        fs::copy("tests/debug-import.js", "tests/debug-import-actual.js")?;
        fs::remove_file("tests/debug-import-actual.js")?;
        Ok(())
    }
}
