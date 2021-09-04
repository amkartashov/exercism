pub fn brackets_are_balanced(string: &str) -> bool {
    // array of bracket pairs: [ (opening, closing) ...]
    const BRACKETS: [(char, char); 3] = [('[', ']'), ('{', '}'), ('(', ')')];

    // vector of expected closing brackets
    //   [ (']',1), ('}',2), (']',3) ] means we expect ...]...]...]...}...}...]...
    let mut expected_close_brackets: Vec<(char, u32)> = Vec::new();

    for c in string.chars() {
        // opening bracket?
        if let Some(&(_, close)) = BRACKETS.iter().find(|(open, _)| *open == c) {
            // do we have unpaired opening bracket atm?
            if let Some(expected) = expected_close_brackets.last_mut() {
                if expected.0 == close {
                    // same type? increase number
                    expected.1 += 1;
                } else {
                    // another type? push new pair
                    expected_close_brackets.push((close, 1));
                }
            } else {
                // we don't expect any atm
                expected_close_brackets.push((close, 1));
            }
        }

        // closing bracket?
        if let Some(&(_, close)) = BRACKETS.iter().find(|(_, close)| *close == c) {
            // do we have unpaired opening bracket atm?
            if let Some(expected) = expected_close_brackets.last_mut() {
                if expected.0 == close {
                    // correct type? decrease and remove if needed
                    expected.1 -= 1;
                    if expected.1 == 0 {
                        expected_close_brackets.pop();
                    }
                } else {
                    // wrong closing bracket
                    return false;
                }
            } else {
                // we don't expect closing bracket here
                return false;
            }
        }
    }

    expected_close_brackets.is_empty()
}
