use crate::token::{Token, TokenType};

pub fn to_html(tokens: &[Token], _source: &str) -> String {
    let mut html = String::new();

    // HTML header
    html.push_str("<!DOCTYPE html>\n");
    html.push_str("<html>\n<head>\n");
    html.push_str("<meta charset=\"UTF-8\">\n");
    html.push_str("<title>Syntax Highlighted Code</title>\n");
    html.push_str(&generate_css());
    html.push_str("</head>\n<body>\n");
    html.push_str("<pre>");

    // Iterate through tokens
    for (i, token) in tokens.iter().enumerate() {
        // Skip EOF token (will be hidden by CSS anyway)
        if token.token_type == TokenType::Eof {
            continue;
        }

        // Add token span
        let class = token_type_to_class(token.token_type);
        html.push_str(&format!(
            "<span class=\"{}\">{}</span>",
            class,
            html_escape(&token.lexeme)
        ));

        // Add spacing
        if i < tokens.len() - 1 {
            let next_token = &tokens[i + 1];
            if next_token.line > token.line {
                // Add newlines for line difference
                for _ in 0..(next_token.line - token.line) {
                    html.push('\n');
                }
            } else if next_token.token_type != TokenType::Eof {
                // Same line, add space
                html.push(' ');
            }
        }
    }

    // Close HTML
    html.push_str("</pre>\n</body>\n</html>");

    html
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn generate_css() -> String {
    r#"<style>
body {
    font-family: 'Courier New', monospace;
    margin: 20px;
    background-color: #f8f9fa;
}
pre {
    background-color: #ffffff;
    border: 1px solid #dee2e6;
    border-radius: 4px;
    padding: 15px;
    line-height: 1.5;
}
.token-number { color: #28a745; font-weight: bold; }
.token-plus, .token-minus, .token-star, .token-slash { color: #dc3545; font-weight: bold; }
.token-equal, .token-equal-equal, .token-bang, .token-bang-equal,
.token-less, .token-less-equal, .token-greater, .token-greater-equal { color: #0066cc; font-weight: bold; }
.token-left-paren, .token-right-paren { color: #917318; font-weight: bold; }
.token-identifier { color: #212529; }
.token-eof { display: none; }
</style>
"#
    .to_string()
}

fn token_type_to_class(token_type: TokenType) -> &'static str {
    match token_type {
        TokenType::Number => "token-number",
        TokenType::Plus => "token-plus",
        TokenType::Minus => "token-minus",
        TokenType::Star => "token-star",
        TokenType::Slash => "token-slash",
        TokenType::Equal => "token-equal",
        TokenType::EqualEqual => "token-equal-equal",
        TokenType::Bang => "token-bang",
        TokenType::BangEqual => "token-bang-equal",
        TokenType::Less => "token-less",
        TokenType::LessEqual => "token-less-equal",
        TokenType::Greater => "token-greater",
        TokenType::GreaterEqual => "token-greater-equal",
        TokenType::LeftParen => "token-left-paren",
        TokenType::RightParen => "token-right-paren",
        TokenType::Identifier => "token-identifier",
        TokenType::Eof => "token-eof",
    }
}
