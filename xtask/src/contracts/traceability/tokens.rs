//! Declaration tokenization for physical contract symbol resolution.

use super::*;

pub(super) fn symbol_resolves(
    path: &Path,
    source: &str,
    symbol: &str,
) -> Result<bool, TraceabilityError> {
    if let Some(pointer) = symbol.strip_prefix('#') {
        return serde_json::from_str::<Value>(source)
            .map_err(|source| TraceabilityError::Json {
                path: path.to_path_buf(),
                source,
            })
            .map(|value| value.pointer(pointer).is_some());
    }

    let tokens = tokenize_declarations(source);
    if let Some((owner, member)) = symbol.split_once("::").or_else(|| symbol.split_once('.')) {
        return Ok(owner_bodies(&tokens, owner)
            .iter()
            .any(|body| member_declared_in_body(&tokens, *body, member)));
    }
    Ok(top_level_item_declared(&tokens, symbol))
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum DeclarationToken<'source> {
    Identifier(&'source str),
    OpenBrace,
    CloseBrace,
    OpenParenthesis,
    CloseParenthesis,
    OpenBracket,
    CloseBracket,
    Semicolon,
    Colon,
    Comma,
    Star,
    Less,
    Greater,
    Other,
}

fn tokenize_declarations(source: &str) -> Vec<DeclarationToken<'_>> {
    let bytes = source.as_bytes();
    let mut tokens = Vec::new();
    let mut index = 0;
    while index < bytes.len() {
        match bytes[index] {
            b if b.is_ascii_whitespace() => index += 1,
            b'/' if bytes.get(index + 1) == Some(&b'/') => {
                index = source[index..]
                    .find('\n')
                    .map_or(bytes.len(), |offset| index + offset + 1);
            }
            b'/' if bytes.get(index + 1) == Some(&b'*') => {
                index = skip_block_comment(bytes, index + 2);
            }
            b'"' => index = skip_quoted(bytes, index + 1, b'"'),
            b'\'' => index = skip_character_literal(bytes, index + 1),
            b'r' if raw_string_end(bytes, index).is_some() => {
                index = raw_string_end(bytes, index).unwrap_or(bytes.len());
            }
            b if is_identifier_start(b) => {
                let start = index;
                index += 1;
                while bytes
                    .get(index)
                    .is_some_and(|byte| is_identifier_continue(*byte))
                {
                    index += 1;
                }
                tokens.push(DeclarationToken::Identifier(&source[start..index]));
            }
            b'{' => {
                tokens.push(DeclarationToken::OpenBrace);
                index += 1;
            }
            b'}' => {
                tokens.push(DeclarationToken::CloseBrace);
                index += 1;
            }
            b'(' => {
                tokens.push(DeclarationToken::OpenParenthesis);
                index += 1;
            }
            b')' => {
                tokens.push(DeclarationToken::CloseParenthesis);
                index += 1;
            }
            b'[' => {
                tokens.push(DeclarationToken::OpenBracket);
                index += 1;
            }
            b']' => {
                tokens.push(DeclarationToken::CloseBracket);
                index += 1;
            }
            b';' => {
                tokens.push(DeclarationToken::Semicolon);
                index += 1;
            }
            b':' => {
                tokens.push(DeclarationToken::Colon);
                index += 1;
            }
            b',' => {
                tokens.push(DeclarationToken::Comma);
                index += 1;
            }
            b'*' => {
                tokens.push(DeclarationToken::Star);
                index += 1;
            }
            b'<' => {
                tokens.push(DeclarationToken::Less);
                index += 1;
            }
            b'>' => {
                tokens.push(DeclarationToken::Greater);
                index += 1;
            }
            _ => {
                tokens.push(DeclarationToken::Other);
                index += 1;
            }
        }
    }
    tokens
}

fn skip_block_comment(bytes: &[u8], mut index: usize) -> usize {
    let mut depth = 1;
    while index < bytes.len() {
        match (bytes[index], bytes.get(index + 1)) {
            (b'/', Some(b'*')) => {
                depth += 1;
                index += 2;
            }
            (b'*', Some(b'/')) => {
                depth -= 1;
                index += 2;
                if depth == 0 {
                    return index;
                }
            }
            _ => index += 1,
        }
    }
    bytes.len()
}

fn skip_quoted(bytes: &[u8], mut index: usize, quote: u8) -> usize {
    while index < bytes.len() {
        if bytes[index] == b'\\' {
            index = index.saturating_add(2);
        } else if bytes[index] == quote {
            return index + 1;
        } else {
            index += 1;
        }
    }
    bytes.len()
}

fn skip_character_literal(bytes: &[u8], index: usize) -> usize {
    let Some(next) = bytes.get(index) else {
        return bytes.len();
    };
    if !is_identifier_start(*next) {
        return skip_quoted(bytes, index, b'\'');
    }

    let mut end = index + 1;
    while bytes
        .get(end)
        .is_some_and(|byte| is_identifier_continue(*byte))
    {
        end += 1;
    }
    if bytes.get(end) == Some(&b'\'') {
        end + 1
    } else {
        index
    }
}

fn raw_string_end(bytes: &[u8], index: usize) -> Option<usize> {
    let mut cursor = index + 1;
    while bytes.get(cursor) == Some(&b'#') {
        cursor += 1;
    }
    if bytes.get(cursor) != Some(&b'"') {
        return None;
    }
    let hashes = cursor - index - 1;
    cursor += 1;
    while cursor < bytes.len() {
        if bytes[cursor] == b'"'
            && bytes
                .get(cursor + 1..cursor + hashes + 1)
                .is_some_and(|suffix| suffix.iter().all(|byte| *byte == b'#'))
        {
            return Some(cursor + hashes + 1);
        }
        cursor += 1;
    }
    Some(bytes.len())
}

const fn is_identifier_start(value: u8) -> bool {
    value.is_ascii_alphabetic() || value == b'_'
}

const fn is_identifier_continue(value: u8) -> bool {
    is_identifier_start(value) || value.is_ascii_digit()
}

fn top_level_item_declared(tokens: &[DeclarationToken<'_>], symbol: &str) -> bool {
    let top_level_depth = if tokens.windows(2).any(|pair| {
        matches!(
            pair,
            [
                DeclarationToken::Identifier("extern"),
                DeclarationToken::OpenBrace
            ]
        )
    }) {
        1
    } else {
        0
    };
    let mut depth = 0_usize;
    for (index, token) in tokens.iter().enumerate() {
        match token {
            DeclarationToken::OpenBrace => depth += 1,
            DeclarationToken::CloseBrace => depth = depth.saturating_sub(1),
            DeclarationToken::Identifier(name) if depth == top_level_depth && *name == symbol => {
                if item_declaration_at(tokens, index) {
                    return true;
                }
            }
            DeclarationToken::Identifier(_)
            | DeclarationToken::OpenParenthesis
            | DeclarationToken::CloseParenthesis
            | DeclarationToken::OpenBracket
            | DeclarationToken::CloseBracket
            | DeclarationToken::Semicolon
            | DeclarationToken::Colon
            | DeclarationToken::Comma
            | DeclarationToken::Star
            | DeclarationToken::Less
            | DeclarationToken::Greater
            | DeclarationToken::Other => {}
        }
    }
    false
}

fn item_declaration_at(tokens: &[DeclarationToken<'_>], index: usize) -> bool {
    matches!(
        tokens.get(index.wrapping_sub(1)),
        Some(DeclarationToken::Identifier(
            "struct" | "enum" | "trait" | "type" | "union" | "mod" | "fn" | "const"
        ))
    ) || matches!(
        tokens.get(index + 1),
        Some(DeclarationToken::OpenParenthesis)
    ) || declaration_since_last_semicolon(tokens, index)
        .contains(&DeclarationToken::Identifier("typedef"))
}

fn owner_bodies(tokens: &[DeclarationToken<'_>], owner: &str) -> Vec<usize> {
    let mut bodies = Vec::new();
    for (index, token) in tokens.iter().enumerate() {
        let is_named_item = matches!(
            token,
            DeclarationToken::Identifier("struct" | "enum" | "trait" | "type" | "union" | "mod")
        ) && matches!(tokens.get(index + 1), Some(DeclarationToken::Identifier(name)) if *name == owner);
        let is_implementation = matches!(token, DeclarationToken::Identifier("impl"))
            && implementation_owner(tokens, index + 1).is_some_and(|name| name == owner);
        if (is_named_item || is_implementation)
            && let Some(opening) = item_body_opening(tokens, index)
            && matching_brace(tokens, opening).is_some()
        {
            bodies.push(opening);
        }
    }
    bodies
}

fn implementation_owner<'source>(
    tokens: &'source [DeclarationToken<'source>],
    mut index: usize,
) -> Option<&'source str> {
    let mut generic_depth = 0_usize;
    let mut first_type = None;
    while let Some(token) = tokens.get(index) {
        match token {
            DeclarationToken::OpenBrace
            | DeclarationToken::CloseBrace
            | DeclarationToken::Semicolon => {
                return first_type;
            }
            DeclarationToken::Less => generic_depth += 1,
            DeclarationToken::Greater => generic_depth = generic_depth.saturating_sub(1),
            DeclarationToken::Identifier("for") if generic_depth == 0 => {
                return tokens.get(index + 1).and_then(identifier);
            }
            DeclarationToken::Identifier(name) if generic_depth == 0 && first_type.is_none() => {
                first_type = Some(*name);
            }
            DeclarationToken::Identifier(_)
            | DeclarationToken::OpenParenthesis
            | DeclarationToken::CloseParenthesis
            | DeclarationToken::OpenBracket
            | DeclarationToken::CloseBracket
            | DeclarationToken::Colon
            | DeclarationToken::Comma
            | DeclarationToken::Star
            | DeclarationToken::Other => {}
        }
        index += 1;
    }
    first_type
}

fn identifier<'source>(token: &DeclarationToken<'source>) -> Option<&'source str> {
    match token {
        DeclarationToken::Identifier(value) => Some(value),
        DeclarationToken::OpenBrace
        | DeclarationToken::CloseBrace
        | DeclarationToken::OpenParenthesis
        | DeclarationToken::CloseParenthesis
        | DeclarationToken::OpenBracket
        | DeclarationToken::CloseBracket
        | DeclarationToken::Semicolon
        | DeclarationToken::Colon
        | DeclarationToken::Comma
        | DeclarationToken::Star
        | DeclarationToken::Less
        | DeclarationToken::Greater
        | DeclarationToken::Other => None,
    }
}

fn item_body_opening(tokens: &[DeclarationToken<'_>], index: usize) -> Option<usize> {
    tokens[index + 1..]
        .iter()
        .position(|token| {
            matches!(
                token,
                DeclarationToken::OpenBrace | DeclarationToken::Semicolon
            )
        })
        .and_then(|offset| {
            let opening = index + offset + 1;
            matches!(tokens.get(opening), Some(DeclarationToken::OpenBrace)).then_some(opening)
        })
}

fn matching_brace(tokens: &[DeclarationToken<'_>], opening: usize) -> Option<usize> {
    let mut depth = 0_usize;
    for (index, token) in tokens.iter().enumerate().skip(opening) {
        match token {
            DeclarationToken::OpenBrace => depth += 1,
            DeclarationToken::CloseBrace => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    return Some(index);
                }
            }
            DeclarationToken::Identifier(_)
            | DeclarationToken::OpenParenthesis
            | DeclarationToken::CloseParenthesis
            | DeclarationToken::OpenBracket
            | DeclarationToken::CloseBracket
            | DeclarationToken::Semicolon
            | DeclarationToken::Colon
            | DeclarationToken::Comma
            | DeclarationToken::Star
            | DeclarationToken::Less
            | DeclarationToken::Greater
            | DeclarationToken::Other => {}
        }
    }
    None
}

fn member_declared_in_body(tokens: &[DeclarationToken<'_>], opening: usize, member: &str) -> bool {
    let Some(closing) = matching_brace(tokens, opening) else {
        return false;
    };
    let mut depth = 0_usize;
    for index in opening + 1..closing {
        match tokens[index] {
            DeclarationToken::OpenBrace => depth += 1,
            DeclarationToken::CloseBrace => depth = depth.saturating_sub(1),
            DeclarationToken::Identifier(name) if depth == 0 && name == member => {
                if member_declaration_at(tokens, index) {
                    return true;
                }
            }
            DeclarationToken::Identifier(_)
            | DeclarationToken::OpenParenthesis
            | DeclarationToken::CloseParenthesis
            | DeclarationToken::OpenBracket
            | DeclarationToken::CloseBracket
            | DeclarationToken::Semicolon
            | DeclarationToken::Colon
            | DeclarationToken::Comma
            | DeclarationToken::Star
            | DeclarationToken::Less
            | DeclarationToken::Greater
            | DeclarationToken::Other => {}
        }
    }
    false
}

fn member_declaration_at(tokens: &[DeclarationToken<'_>], index: usize) -> bool {
    matches!(
        tokens.get(index.wrapping_sub(1)),
        Some(DeclarationToken::Identifier("fn" | "const" | "type"))
    ) || matches!(
        tokens.get(index + 1),
        Some(
            DeclarationToken::Colon
                | DeclarationToken::Semicolon
                | DeclarationToken::OpenBrace
                | DeclarationToken::OpenBracket
                | DeclarationToken::OpenParenthesis
        )
    ) || matches!(
        (tokens.get(index.wrapping_sub(1)), tokens.get(index + 1)),
        (
            Some(DeclarationToken::Star),
            Some(DeclarationToken::CloseParenthesis)
        ) | (
            Some(DeclarationToken::OpenBrace | DeclarationToken::Comma),
            Some(DeclarationToken::Comma)
        )
    )
}

fn declaration_since_last_semicolon<'source>(
    tokens: &'source [DeclarationToken<'source>],
    index: usize,
) -> &'source [DeclarationToken<'source>] {
    let start = tokens[..index]
        .iter()
        .rposition(|token| {
            matches!(
                token,
                DeclarationToken::Semicolon | DeclarationToken::CloseBrace
            )
        })
        .map_or(0, |position| position + 1);
    &tokens[start..index]
}
