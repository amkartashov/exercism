pub fn brackets_are_balanced(string: &str) -> bool {
    // vector of expected closing brackets
    let mut expected_close_brackets = Vec::new();

    macro_rules! brackets_stack_check {
        ($stack:ident, $c:ident, $($open:literal $close:literal),+ $(,)?) => {
            match $c {
                $(
                    $open => $stack.push($close),
                    $close => {
                        if let Some(expected) = $stack.pop() {
                            if expected != $close {
                                // wrong closing bracket
                                return false;
                            }
                        } else {
                            // we don't expect closing bracket here
                            return false;
                        };
                    },
                )*
                _ => {},
            }
        };
    }

    for c in string.chars() {
        brackets_stack_check!(
            expected_close_brackets,
            c,
            '{' '}',
            '(' ')',
            '[' ']',
        );
    }

    expected_close_brackets.is_empty()
}
