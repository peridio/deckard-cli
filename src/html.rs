/// HTML minification functionality
pub fn minify(html: &str) -> String {
    let mut result = String::new();
    let mut prev_char = ' ';
    let mut in_tag = false;
    let mut in_quotes = false;
    let mut quote_char = ' ';

    for ch in html.chars() {
        match ch {
            '<' if !in_quotes => {
                in_tag = true;
                // Remove whitespace before tag
                if !result.is_empty() && prev_char.is_whitespace() {
                    let trimmed_len = result.trim_end().len();
                    result.truncate(trimmed_len);
                }
                result.push(ch);
            }
            '>' if !in_quotes => {
                in_tag = false;
                result.push(ch);
            }
            '"' | '\'' if in_tag => {
                if in_quotes && ch == quote_char {
                    in_quotes = false;
                } else if !in_quotes {
                    in_quotes = true;
                    quote_char = ch;
                }
                result.push(ch);
            }
            ' ' | '\t' | '\n' | '\r' if !in_quotes => {
                // Collapse multiple whitespaces to single space
                if !prev_char.is_whitespace() {
                    result.push(' ');
                }
            }
            _ => {
                result.push(ch);
            }
        }

        prev_char = ch;
    }

    result.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_minification() {
        let html = r#"
            <html>
                <head>
                    <title>Test</title>
                </head>
                <body>
                    <h1>Hello World</h1>
                </body>
            </html>
        "#;

        let minified = minify(html);
        assert!(!minified.contains("\n"));
        assert!(minified.contains("<html><head><title>Test</title></head>"));
        assert!(minified.contains("<body><h1>Hello World</h1></body></html>"));
    }

    #[test]
    fn test_preserve_spaces_in_quotes() {
        let html = r#"<div class="my   class" id="test   id">Content</div>"#;
        let minified = minify(html);
        assert_eq!(
            minified,
            r#"<div class="my   class" id="test   id">Content</div>"#
        );
    }

    #[test]
    fn test_remove_whitespace_between_tags() {
        let html = r#"<div>   <span>Text</span>   <p>More</p>   </div>"#;
        let minified = minify(html);
        assert_eq!(minified, "<div><span>Text</span><p>More</p></div>");
    }

    #[test]
    fn test_single_quotes() {
        let html = r#"<div class='my class' data-value='test  value'>Content</div>"#;
        let minified = minify(html);
        assert_eq!(
            minified,
            r#"<div class='my class' data-value='test  value'>Content</div>"#
        );
    }

    #[test]
    fn test_mixed_quotes() {
        let html = r#"<div onclick="alert('Hello  World')">Click</div>"#;
        let minified = minify(html);
        assert_eq!(
            minified,
            r#"<div onclick="alert('Hello  World')">Click</div>"#
        );
    }

    #[test]
    fn test_empty_html() {
        assert_eq!(minify(""), "");
        assert_eq!(minify("   "), "");
        assert_eq!(minify("\n\t\r"), "");
    }

    #[test]
    fn test_text_only() {
        let html = "Just some text without tags";
        assert_eq!(minify(html), "Just some text without tags");
    }

    #[test]
    fn test_self_closing_tags() {
        let html = r#"<img src="test.jpg" />  <br />  <hr />"#;
        let minified = minify(html);
        assert_eq!(minified, r#"<img src="test.jpg" /><br /><hr />"#);
    }
}
