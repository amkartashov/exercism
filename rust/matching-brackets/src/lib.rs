pub fn brackets_are_balanced(string: &str) -> bool {
    // array of bracket pairs: [ (opening, closing) ...]
    const BRACKETS: [(char, char); 3] = [('[', ']'), ('{', '}'), ('(', ')')];

    // vector of expected closing brackets
    let mut expected_close_brackets = Vec::new();

    for c in string.chars() {
        for (open, close) in BRACKETS {
            match c {
                o if o == open => expected_close_brackets.push(close),
                c if c == close => {
                    // check that this is what we expect
                    if let Some(expected) = expected_close_brackets.pop() {
                        if expected != close {
                            // wrong closing bracket
                            return false;
                        }
                    } else {
                        // we don't expect closing bracket here
                        return false;
                    };
                }
                _ => {}
            };
        }
    }

    expected_close_brackets.is_empty()
}
