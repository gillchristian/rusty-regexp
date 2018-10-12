fn main() {
    println!("match_char('.', 'e') -> {}", match_char('.', 'e'));
    println!("match_char('a', 'e') -> {}", match_char('a', 'e'));
    println!("match_char('a', 'a') -> {}", match_char('a', 'a'));

    println!();

    println!("regexp_match(\"$\", \"\") -> {}", regexp_match("$", ""));

    println!();

    println!(
        "regexp_match(\"ab\", \"ab\") -> {}",
        regexp_match("ab", "ab")
    );

    println!();

    println!(
        "regexp_match(\"ab\", \"abc\") -> {}",
        regexp_match("ab", "abc")
    );

    println!();

    println!(
        "regexp_match(\"ab\", \"cabc\") -> {}",
        regexp_match("ab", "cabc")
    );

    println!();

    println!(
        "regexp_match(\"a?b\", \"b\") -> {}",
        regexp_match("a?b", "b")
    );
    println!(
        "regexp_match(\"a?b\", \"ab\") -> {}",
        regexp_match("a?b", "ab")
    );
    println!(
        "regexp_match(\"a?b\", \"ac\") -> {}",
        regexp_match("a?b", "ac")
    );

    println!();

    println!(
        "regexp_match(\"^ab\", \"ab\") -> {}",
        regexp_match("^ab", "ab")
    );
}

fn match_char(p: char, c: char) -> bool {
    return p == '.' || p == c;
}

// ---

fn regexp_match(pattern: &str, text: &str) -> bool {
    if pattern.chars().next().unwrap() == '^' {
        return match_(&pattern[1..], text);
    }

    if text == "" {
        return pattern == "$";
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

    let p_fst = match char_at(pattern, 0) {
        Some(char) => char,
        None => return false,
    };
    let t_fst = match char_at(text, 0) {
        Some(char) => char,
        None => return false,
    };

    if pattern.len() == 1 {
        return match_char(p_fst, t_fst);
    }

    let p_snd = match char_at(pattern, 1) {
        Some(char) => char,
        None => return false,
    };

    if p_snd == '?' {
        return match_question(pattern, text);
    }

    if p_snd == '*' {
        return match_pattern(pattern, text);
    }

    return match_char(p_fst, t_fst) && match_(&pattern[1..], &text[1..]);
}

fn match_question(pattern: &str, text: &str) -> bool {
    let p_fst = match char_at(pattern, 0) {
        Some(char) => char,
        None => return false, // ???
    };
    let t_fst = match char_at(text, 0) {
        Some(char) => char,
        None => return false, // ???
    };
    let stripped = &pattern[2..];

    return (match_char(p_fst, t_fst) && match_(stripped, &text[1..])) || match_(stripped, text);
}

fn match_pattern(pattern: &str, text: &str) -> bool {
    let p_fst = match char_at(pattern, 0) {
        Some(char) => char,
        None => return false, // ???
    };
    let t_fst = match char_at(text, 0) {
        Some(char) => char,
        None => return false, // ???
    };

    return (match_char(p_fst, t_fst) && match_(pattern, &text[1..])) || match_(&pattern[2..], text);
}

fn char_at(str: &str, i: usize) -> Option<char> {
    match str.chars().nth(i) {
        Some(char) => Some(char),
        None => None,
    }
}
