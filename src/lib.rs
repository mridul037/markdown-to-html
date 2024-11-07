use wasm_bindgen::prelude::*;
use pulldown_cmark::{html, Parser};

#[wasm_bindgen]
pub fn convert_markdown_to_html(markdown_input: &str) -> String {
    let parser = Parser::new(markdown_input);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}
