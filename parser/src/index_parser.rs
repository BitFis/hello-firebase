use pest::iterators::Pair;
use pest::Parser;
use regex::Regex;
use std::fs;
use std::vec::Vec;

#[derive(pest_derive::Parser)]
#[grammar = "index.pest"]
struct IndexParser;

pub fn parse(file: String, scripts: Vec<String>) -> std::io::Result<()> {
    let contents = fs::read_to_string(file.clone()).expect("Provided file can not be read");

    let mut inject: Vec<String> = Vec::new();
    for script in scripts {
        let con = fs::read_to_string(script)?;
        inject.push(con);
    }

    fs::write(file, parse_content(contents, inject))?;
    Ok(())
}

fn process_source(source: &String) -> String {
    return String::from(format!("`{}${{importquery}}`", source));
}

fn verify_source(source: &String) -> bool {
    let re = Regex::new(r"^https?://").unwrap();
    return !re.is_match(&source);
}

fn parse_script(tag: &str, parse: Pair<Rule>) -> String {
    let mut output = String::from(format!("<script>appendElement(\"{}\", {{", tag));

    let mut process_source_tag = false;
    let mut skip_this_tag = false;

    let content = String::from(parse.as_str());
    for pair in parse.into_inner() {
        match pair.as_rule() {
            Rule::tag => {
                let tag = pair.as_str();
                if tag.eq("src") || tag.eq("href") {
                    process_source_tag = true
                }
                output.push_str(format!("\"{}\"", tag).as_str());
            }
            Rule::value => {
                let value = String::from(pair.as_str());
                if process_source_tag {
                    process_source_tag = false;
                    output.push_str(format!(":{},", process_source(&value)).as_str());

                    if !verify_source(&value) {
                        skip_this_tag = true;
                        break;
                    }

                    continue;
                }
                output.push_str(format!(":\"{}\",", value).as_str());
            }
            _ => {
                println!("unknown script match")
            }
        }
    }

    if skip_this_tag {
        return content;
    }

    output.push_str("});</script>");

    return output;
}

const SCRIPT_TAG_START: &str = "<script type=\"text/javascript\">";
const SCRIPT_TAG_END: &str = "</script>";

fn parse_content(content: String, inject_scripts: Vec<String>) -> String {
    let parse = IndexParser::parse(Rule::main, &content)
        .unwrap_or_else(|e| panic!("{}", e))
        .next()
        .unwrap();

    let mut output = String::from("");

    for pair in parse.into_inner() {
        match pair.as_rule() {
            Rule::script => {
                output.push_str(&parse_script("script", pair));
            }
            Rule::head => {
                output.push_str(pair.as_str());
                if inject_scripts.len() > 0 {
                    output.push_str(SCRIPT_TAG_START);
                    output.push_str(
                        inject_scripts
                            .join(format!("{}{}", SCRIPT_TAG_END, SCRIPT_TAG_START).as_str())
                            .as_str(),
                    );
                    output.push_str(SCRIPT_TAG_END);
                }
            }
            Rule::link => {
                output.push_str(&parse_script("link", pair));
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
    fn parse_retrieve_js_module() {
        let input = String::from(
            r#"
<body>
  <app-root></app-root>
  <script type="module" src="polyfills-B5TEDK75.js"></script>
  <script src="main-CDZAXZJF.js" type="module"></script>
</body>
        "#,
        );
        let expect = String::from(
            r#"
<body>
  <app-root></app-root>
  <script>appendElement("script", {"type":"module","src":`polyfills-B5TEDK75.js${importquery}`,});</script>
  <script>appendElement("script", {"src":`main-CDZAXZJF.js${importquery}`,"type":"module",});</script>
</body>
        "#,
        );
        assert_eq!(trim(parse_content(input, Vec::new())), trim(expect));
    }

    #[test]
    fn parse_retrieve_css_skip_noscript_and_global_link() {
        let input = String::from(
            r#"
<head>
  <link rel="stylesheet" href="styles-JJX5A6AV.css" media="print" onload="this.media='all'"><noscript>
    <link rel="stylesheet" href="styles-JJX5A6AV.css"></noscript>
</head>
<body>
  <app-root></app-root>
  <link rel="modulepreload" href="chunk-2RG4V45Z.js">
</body>
        "#,
        );
        let expect = String::from(
            r#"
<head>
  <script>appendElement("link", {"rel":"stylesheet","href":`styles-JJX5A6AV.css${importquery}`,"media":"print","onload":"this.media='all'",});</script><noscript>
    <link rel="stylesheet" href="styles-JJX5A6AV.css"></noscript>
</head>
<body>
  <app-root></app-root>
  <script>appendElement("link", {"rel":"modulepreload","href":`chunk-2RG4V45Z.js${importquery}`,});</script>
</body>
        "#,
        );
        assert_eq!(trim(parse_content(input, Vec::new())), trim(expect));
    }

    #[test]
    fn parse_retrieve_modulepreload_link() {
        let input = String::from(
            r#"
<body>
  <app-root></app-root>
  <link rel="modulepreload" href="chunk-2RG4V45Z.js">
</body>
        "#,
        );
        let expect = String::from(
            r#"
<body>
  <app-root></app-root>
  <script>appendElement("link", {"rel":"modulepreload","href":`chunk-2RG4V45Z.js${importquery}`,});</script>
</body>
        "#,
        );
        assert_eq!(trim(parse_content(input, Vec::new())), trim(expect));
    }

    #[test]
    fn parse_ignore_global_link() {
        let input = String::from(
            r#"
<head>
  <link href="http://cdn.jsdelivr.net/npm/bootstrap@5.2.3/dist/css/bootstrap.min.css" rel="stylesheet"
    crossorigin="anonymous" integrity="sha384-rbsA2VBKQhggwzxH7pPCaAqO46MgnOM80zW1RWuH61DGLwZJEdK2Kadq2F9CUG65" />
</head>
        "#,
        );
        assert_eq!(trim(parse_content(input.clone(), Vec::new())), trim(input));
    }

    #[test]
    fn append_custom_script() {
        let input = String::from(
            r#"
<head>
</head>
        "#,
        );
        let expect = String::from(
            r#"
<head>
<script type="text/javascript">const val=12;</script>
<script type="text/javascript">const other=12;</script>
</head>
        "#,
        );
        assert_eq!(
            trim(parse_content(
                input.clone(),
                vec![
                    String::from("const val=12;"),
                    String::from("const other=12;"),
                ]
            )),
            trim(expect)
        );
    }
}
