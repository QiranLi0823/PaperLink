//! PaperML Parser
//!
//! Parses PaperML text into an AST using pest.

use pest::Parser;
use pest_derive::Parser;
use thiserror::Error;

use crate::ast::*;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct PaperMLParser;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Parse error: {0}")]
    PestError(#[from] pest::error::Error<Rule>),
    #[error("Invalid syntax: {0}")]
    InvalidSyntax(String),
}

/// Parse PaperML text into a Document AST
pub fn parse(input: &str) -> Result<Document, ParseError> {
    let pairs = PaperMLParser::parse(Rule::document, input)?;
    let mut doc = Document::new();

    for pair in pairs {
        match pair.as_rule() {
            Rule::document => {
                for inner in pair.into_inner() {
                    if let Some(block) = parse_block(inner)? {
                        match block {
                            ParsedItem::Block(b) => doc.content.push(b),
                            ParsedItem::Meta(m) => doc.meta = Some(m),
                            ParsedItem::Abstract(text) => {
                                if let Some(ref mut meta) = doc.meta {
                                    meta.abstract_text = Some(text);
                                } else {
                                    doc.meta = Some(Meta {
                                        title: None,
                                        authors: Vec::new(),
                                        abstract_text: Some(text),
                                    });
                                }
                            }
                        }
                    }
                }
            }
            Rule::EOI => {}
            _ => {}
        }
    }

    Ok(doc)
}

enum ParsedItem {
    Block(Block),
    Meta(Meta),
    Abstract(String),
}

fn parse_block(pair: pest::iterators::Pair<Rule>) -> Result<Option<ParsedItem>, ParseError> {
    match pair.as_rule() {
        Rule::meta_block => Ok(Some(ParsedItem::Meta(parse_meta(pair)?))),
        Rule::abstract_block => Ok(Some(ParsedItem::Abstract(parse_abstract(pair)?))),
        Rule::section => Ok(Some(ParsedItem::Block(Block::Section(parse_section(
            pair, 1,
        )?)))),
        Rule::subsection => Ok(Some(ParsedItem::Block(Block::Section(parse_section(
            pair, 2,
        )?)))),
        Rule::subsubsection => Ok(Some(ParsedItem::Block(Block::Section(parse_section(
            pair, 3,
        )?)))),
        Rule::figure => Ok(Some(ParsedItem::Block(Block::Figure(parse_figure(pair)?)))),
        Rule::table => Ok(Some(ParsedItem::Block(Block::Table(parse_table(pair)?)))),
        Rule::equation => Ok(Some(ParsedItem::Block(Block::Equation(parse_equation(
            pair,
        )?)))),
        Rule::paragraph => Ok(Some(ParsedItem::Block(Block::Paragraph(parse_paragraph(
            pair,
        )?)))),
        Rule::EOI => Ok(None),
        _ => Ok(None),
    }
}

fn parse_meta(pair: pest::iterators::Pair<Rule>) -> Result<Meta, ParseError> {
    let mut meta = Meta::default();

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::meta_attrs => {
                for attr in inner.into_inner() {
                    match attr.as_rule() {
                        Rule::meta_attr => {
                            for attr_inner in attr.into_inner() {
                                match attr_inner.as_rule() {
                                    Rule::meta_title => {
                                        meta.title = Some(extract_quoted_string(attr_inner)?);
                                    }
                                    Rule::meta_authors => {
                                        meta.authors = extract_string_array(attr_inner)?;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    Ok(meta)
}

fn parse_abstract(pair: pest::iterators::Pair<Rule>) -> Result<String, ParseError> {
    for inner in pair.into_inner() {
        if inner.as_rule() == Rule::abstract_content {
            return Ok(inner.as_str().trim().to_string());
        }
    }
    Ok(String::new())
}

fn parse_section(pair: pest::iterators::Pair<Rule>, level: u8) -> Result<Section, ParseError> {
    let mut section = Section::new(level, "");

    for inner in pair.into_inner() {
        if inner.as_rule() == Rule::section_title {
            section.title = inner.as_str().trim().to_string();
        }
    }

    Ok(section)
}

fn parse_figure(pair: pest::iterators::Pair<Rule>) -> Result<Figure, ParseError> {
    let mut figure = Figure::new("");

    for inner in pair.into_inner() {
        if inner.as_rule() == Rule::figure_attrs {
            for attr in inner.into_inner() {
                if attr.as_rule() == Rule::figure_attr {
                    for attr_inner in attr.into_inner() {
                        match attr_inner.as_rule() {
                            Rule::figure_path => {
                                figure.path = extract_quoted_string(attr_inner)?;
                            }
                            Rule::figure_caption => {
                                figure.caption = Some(extract_quoted_string(attr_inner)?);
                            }
                            Rule::figure_label => {
                                figure.label = Some(extract_quoted_string(attr_inner)?);
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    Ok(figure)
}

fn parse_table(pair: pest::iterators::Pair<Rule>) -> Result<Table, ParseError> {
    let mut table = Table::new();

    for inner in pair.into_inner() {
        if inner.as_rule() == Rule::table_attrs {
            for attr in inner.into_inner() {
                if attr.as_rule() == Rule::table_attr {
                    for attr_inner in attr.into_inner() {
                        match attr_inner.as_rule() {
                            Rule::table_caption => {
                                table.caption = Some(extract_quoted_string(attr_inner)?);
                            }
                            Rule::table_label => {
                                table.label = Some(extract_quoted_string(attr_inner)?);
                            }
                            Rule::table_columns => {
                                table.columns = extract_string_array(attr_inner)?;
                            }
                            Rule::table_rows => {
                                table.rows = extract_rows_array(attr_inner)?;
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    Ok(table)
}

fn parse_equation(pair: pest::iterators::Pair<Rule>) -> Result<Equation, ParseError> {
    let mut equation = Equation::new("");

    for inner in pair.into_inner() {
        if inner.as_rule() == Rule::equation_attrs {
            for attr in inner.into_inner() {
                if attr.as_rule() == Rule::equation_attr {
                    for attr_inner in attr.into_inner() {
                        match attr_inner.as_rule() {
                            Rule::equation_content => {
                                equation.content = extract_quoted_string(attr_inner)?;
                            }
                            Rule::equation_label => {
                                equation.label = Some(extract_quoted_string(attr_inner)?);
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    Ok(equation)
}

fn parse_paragraph(pair: pest::iterators::Pair<Rule>) -> Result<Paragraph, ParseError> {
    let mut inlines = Vec::new();

    for inner in pair.into_inner() {
        if inner.as_rule() == Rule::paragraph_content {
            for elem in inner.into_inner() {
                match elem.as_rule() {
                    Rule::plain_text => {
                        inlines.push(Inline::Text {
                            value: elem.as_str().to_string(),
                        });
                    }
                    Rule::citation => {
                        for cite_inner in elem.into_inner() {
                            if cite_inner.as_rule() == Rule::cite_key {
                                inlines.push(Inline::Citation {
                                    key: cite_inner.as_str().to_string(),
                                });
                            }
                        }
                    }
                    Rule::reference => {
                        for ref_inner in elem.into_inner() {
                            if ref_inner.as_rule() == Rule::ref_target {
                                inlines.push(Inline::Reference {
                                    target: ref_inner.as_str().to_string(),
                                });
                            }
                        }
                    }
                    Rule::inline_math => {
                        for math_inner in elem.into_inner() {
                            if math_inner.as_rule() == Rule::math_content {
                                inlines.push(Inline::Math {
                                    content: math_inner.as_str().to_string(),
                                });
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(Paragraph::new(inlines))
}

fn extract_quoted_string(pair: pest::iterators::Pair<Rule>) -> Result<String, ParseError> {
    for inner in pair.into_inner() {
        if inner.as_rule() == Rule::quoted_string {
            for s in inner.into_inner() {
                if s.as_rule() == Rule::inner_string {
                    return Ok(s.as_str().to_string());
                }
            }
        }
    }
    Ok(String::new())
}

fn extract_string_array(pair: pest::iterators::Pair<Rule>) -> Result<Vec<String>, ParseError> {
    let mut result = Vec::new();

    for inner in pair.into_inner() {
        if inner.as_rule() == Rule::string_array {
            for item in inner.into_inner() {
                if item.as_rule() == Rule::quoted_string {
                    for s in item.into_inner() {
                        if s.as_rule() == Rule::inner_string {
                            result.push(s.as_str().to_string());
                        }
                    }
                }
            }
        }
    }

    Ok(result)
}

fn extract_rows_array(pair: pest::iterators::Pair<Rule>) -> Result<Vec<Vec<String>>, ParseError> {
    let mut result = Vec::new();

    for inner in pair.into_inner() {
        if inner.as_rule() == Rule::rows_array {
            for row in inner.into_inner() {
                if row.as_rule() == Rule::string_array {
                    let mut row_values = Vec::new();
                    for item in row.into_inner() {
                        if item.as_rule() == Rule::quoted_string {
                            for s in item.into_inner() {
                                if s.as_rule() == Rule::inner_string {
                                    row_values.push(s.as_str().to_string());
                                }
                            }
                        }
                    }
                    result.push(row_values);
                }
            }
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_section() {
        let input = "@section Introduction\n";

        // Debug: try parsing just section rule first
        let section_result = PaperMLParser::parse(Rule::section, input);
        eprintln!("Section rule parse result: {:?}", section_result);

        let doc = parse(input).unwrap();
        eprintln!("Parsed content: {:?}", doc.content);
        assert_eq!(doc.content.len(), 1, "Expected 1 block, got {:?}", doc.content);
        if let Block::Section(s) = &doc.content[0] {
            assert_eq!(s.level, 1);
            assert_eq!(s.title, "Introduction");
        } else {
            panic!("Expected Section, got {:?}", doc.content[0]);
        }
    }

    #[test]
    fn test_parse_paragraph_with_citation() {
        let input = "This is a test with @cite{ref2024} citation.\n";
        let doc = parse(input).unwrap();
        assert_eq!(doc.content.len(), 1);
        if let Block::Paragraph(p) = &doc.content[0] {
            assert_eq!(p.content.len(), 3);
        } else {
            panic!("Expected Paragraph");
        }
    }

    #[test]
    fn test_parse_figure() {
        let input = r#"@figure{
  path = "test.png"
  caption = "Test figure"
  label = "fig:test"
}"#;
        let doc = parse(input).unwrap();
        assert_eq!(doc.content.len(), 1);
        if let Block::Figure(f) = &doc.content[0] {
            assert_eq!(f.path, "test.png");
            assert_eq!(f.caption, Some("Test figure".to_string()));
            assert_eq!(f.label, Some("fig:test".to_string()));
        } else {
            panic!("Expected Figure");
        }
    }
}
