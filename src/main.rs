fn main() {
    println!(
        "regexp_match(\"a*b\", \"aaaaaaaaaaaaab\") -> {}",
        regexp_match("a*b", "aaaaaaaaaaaaab")
    );
}

fn regexp_match(pattern: &str, text: &str) -> bool {
    if text == "" {
        return pattern == "$";
    }

    if pattern.chars().nth(0).unwrap() == '^' {
        return match_(&pattern[1..], text);
    }

    return match_(&[".*", pattern].join(""), text);
}

fn match_(pattern: &str, text: &str) -> bool {
    if pattern == text || pattern == "" {
        return true;
    }
    if text == "" {
        return pattern == "$";
    }

    // at this point we know pattern & text have at least one character
    let p_fst = pattern.chars().nth(0).unwrap();
    let t_fst = text.chars().nth(0).unwrap();

    // try to get the 2nd character (might be '*' or '?')
    let p_snd = match pattern.chars().nth(1) {
        Some(char) => char,
        // only one char in our pattern, let's match that
        None => return match_char(p_fst, t_fst),
    };

    if p_snd == '?' {
        return match_question(pattern, text);
    }
    if p_snd == '*' {
        return match_pattern(pattern, text);
    }

    // match the first character and the rest
    return match_char(p_fst, t_fst) && match_(&pattern[1..], &text[1..]);
}

fn match_question(pattern: &str, text: &str) -> bool {
    let p_fst = pattern.chars().nth(0).unwrap();
    let t_fst = text.chars().nth(0).unwrap();
    let stripped = &pattern[2..];

    // it can either match the optional character so we need to also match the rest (without the 'x?')
    // or not, and then we also need to match the rest (without the 'x?')
    return (match_char(p_fst, t_fst) && match_(stripped, &text[1..])) || match_(stripped, text);
}

fn match_pattern(pattern: &str, text: &str) -> bool {
    let p_fst = pattern.chars().nth(0).unwrap();
    let t_fst = text.chars().nth(0).unwrap();

    // it can either match the character so we need to also match the rest (but don't remove the 'x*')
    // or not, and then we also need to match the rest (and remove the 'x*')
    return (match_char(p_fst, t_fst) && match_(pattern, &text[1..])) || match_(&pattern[2..], text);
}

fn match_char(p: char, c: char) -> bool {
    // '.' matches all, otherwise pattern and char should be equal
    return p == '.' || p == c;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regexp_match() {
        assert_eq!(regexp_match("a*b", "aaaaab"), true);
        assert_eq!(regexp_match("$", ""), true);
        assert_eq!(regexp_match("ab", "ab"), true);
        assert_eq!(regexp_match("ab", "abc"), true);
        assert_eq!(regexp_match("ab", "cabc"), true);
        assert_eq!(regexp_match("a?b", "b"), true);
        assert_eq!(regexp_match("a?b", "ab"), true);
        assert_eq!(regexp_match("^ab", "ab"), true);
        assert_eq!(regexp_match("a", "abcdefghi"), true);
        assert_eq!(regexp_match("b", "abcdefghi"), true);
        assert_eq!(regexp_match("c", "abcdefghi"), true);
        assert_eq!(regexp_match("i", "abcdefghi"), true);

        assert_eq!(regexp_match("a?b", "ac"), false);
        assert_eq!(regexp_match("abc", "a"), false);
        assert_eq!(regexp_match("^ab", "zab"), false);
    }

    #[test]
    fn test_match_char() {
        assert_eq!(match_char('.', 'e'), true);
        assert_eq!(match_char('a', 'a'), true);

        assert_eq!(match_char('a', 'e'), false);
    }
}
