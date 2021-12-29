use super::*;

#[test]
fn test_layout_trace() {
    let (config, mut builder) = pre_config();
    let mode = CssInlineMode::None;
    let (html, css) = config.compile_html(include_str!("layout.html"), &mut builder, &mode).unwrap();
    std::fs::write("tests/html/layout/layout.traced.html", html.as_bytes()).unwrap();
    std::fs::write("tests/html/layout/layout.traced.css", css.as_bytes()).unwrap();
    assert_eq!(html, include_str!("layout.traced.html"));
    assert_eq!(css, include_str!("layout.traced.css"));
}

#[test]
fn test_layout_inline() {
    let (config, mut builder) = pre_config();
    let mode = CssInlineMode::Inline;
    let (html, css) = config.compile_html(include_str!("layout.html"), &mut builder, &mode).unwrap();
    std::fs::write("tests/html/layout/layout.inline.html", html.as_bytes()).unwrap();
    std::fs::write("tests/html/layout/layout.inline.css", css.as_bytes()).unwrap();
    assert_eq!(html, include_str!("layout.inline.html"));
    assert_eq!(css, include_str!("layout.inline.css"));
}
