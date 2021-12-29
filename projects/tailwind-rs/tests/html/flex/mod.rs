use super::*;

#[test]
fn test_flex_trace() {
    let (config, mut builder) = pre_config();
    let mode = CssInlineMode::None;
    let (html, css) = config.compile_html(include_str!("flex.html"), &mut builder, &mode).unwrap();
    std::fs::write("tests/html/flex/flex.traced.html", html.as_bytes()).unwrap();
    std::fs::write("tests/html/flex/flex.traced.css", css.as_bytes()).unwrap();
    assert_eq!(html, include_str!("flex.traced.html"));
    assert_eq!(css, include_str!("flex.traced.css"));
}

#[test]
fn test_flex_inline() {
    let (config, mut builder) = pre_config();
    let mode = CssInlineMode::Inline;
    let (html, css) = config.compile_html(include_str!("flex.html"), &mut builder, &mode).unwrap();
    std::fs::write("tests/html/flex/flex.inline.html", html.as_bytes()).unwrap();
    std::fs::write("tests/html/flex/flex.inline.css", css.as_bytes()).unwrap();
    assert_eq!(html, include_str!("flex.inline.html"));
    assert_eq!(css, include_str!("flex.inline.css"));
}
