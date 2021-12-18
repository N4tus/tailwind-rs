use super::*;

#[test]
fn test_layout_trace() {
    let mut config = GlobalConfig::default();
    config.css.minify = false;
    config.tailwind.preflight.disable = true;
    let (html, css) = config.compile_html_traced(include_str!("layout.html")).unwrap();
    // std::fs::write("src/support/support_html/test/layout/layout.traced.html", html.as_bytes()).unwrap();
    // std::fs::write("src/support/support_html/test/layout/layout.traced.css", css.as_bytes()).unwrap();
    assert_eq!(html, include_str!("layout.traced.html"));
    assert_eq!(css, include_str!("layout.traced.css"));
}

#[test]
fn test_layout_inline() {
    let mut config = GlobalConfig::default();
    config.css.minify = false;
    config.tailwind.preflight.disable = true;
    let (html, css) = config.compile_html_inline(include_str!("layout.html")).unwrap();
    // std::fs::write("src/support/support_html/test/layout/layout.inline.html", html.as_bytes()).unwrap();
    // std::fs::write("src/support/support_html/test/layout/layout.inline.css", css.as_bytes()).unwrap();
    assert_eq!(html, include_str!("layout.inline.html"));
    assert_eq!(css, include_str!("layout.inline.css"));
}

#[test]
fn test_layout_scoped() {
    let mut config = GlobalConfig::default();
    config.css.minify = false;
    config.tailwind.preflight.disable = true;
    let (html, css) = config.compile_html_scoped(include_str!("layout.html")).unwrap();
    // std::fs::write("src/support/support_html/test/layout/layout.scoped.html", html.as_bytes()).unwrap();
    // std::fs::write("src/support/support_html/test/layout/layout.scoped.css", css.as_bytes()).unwrap();
    assert_eq!(html, include_str!("layout.scoped.html"));
    assert_eq!(css, include_str!("layout.scoped.css"));
}