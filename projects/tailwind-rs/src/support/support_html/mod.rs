use log::error;
use tl::{parse, Bytes, Node, ParserOptions};

use tailwind_css::{InlineMode, TailwindBuilder};

use crate::{config::HtmlConfig, Result};

#[cfg(test)]
mod test;

impl HtmlConfig {
    pub fn trace_all_class(input: &str, tw: &mut TailwindBuilder) -> Result<String> {
        let mut dom = parse(input, ParserOptions::default())?;
        for node in dom.nodes_mut() {
            // ignore if any problem
            trace_class(node, tw);
        }
        Ok(dom.inner_html())
    }
    pub fn inline_all_class(input: &str, tw: &mut TailwindBuilder) -> Result<String> {
        let mut dom = parse(input, ParserOptions::default())?;
        for node in dom.nodes_mut() {
            // ignore if any problem
            inline_class(node, tw);
        }
        Ok(dom.inner_html())
    }
}

fn trace_class(node: &mut Node, tw: &mut TailwindBuilder) -> Option<()> {
    let attributes = node.as_tag_mut()?.attributes_mut();
    let class = attributes.get_mut("class")??;
    match tw.interpret(class.try_as_utf8_str()?, None) {
        Ok(o) => {
            class.set(o).ok()??;
        },
        Err(e) => error!("{}", e),
    }
    Some(())
}

fn inline_class(node: &mut Node, tw: &mut TailwindBuilder) -> Option<()> {
    let attributes = node.as_tag_mut()?.attributes_mut();
    let class = attributes.get_mut("class")??;
    let mut style = Bytes::new();
    match tw.interpret(class.try_as_utf8_str()?, Some(InlineMode::Inline)) {
        Ok(o) => {
            class.set(o.get_class()).ok()??;
            style.set(o.get_style()).ok()??;
        },
        Err(e) => {
            error!("{}", e);
            return Some(());
        },
    };
    attributes.insert("style", Some(style));
    Some(())
}