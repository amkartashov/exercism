pub fn encode(source: &str) -> String {
    if source.is_empty() {
        return String::from("");
    };

    let mut result = String::new();
    let mut chars = source.chars();
    let mut counter = 1;
    let mut char = chars.next().unwrap();

    for next_char in chars {
        if next_char == char {
            counter += 1;
        } else {
            if counter > 1 {
                result += counter.to_string().as_str();
            };
            result.push(char);
            counter = 1;
            char = next_char;
        };
    }

    // write last char
    if counter > 1 {
        result += counter.to_string().as_str();
    }
    result.push(char);

    result
}

pub fn decode(source: &str) -> String {
    if source.is_empty() {
        return String::from("");
    };

    let mut result = String::new();
    let mut counter = 0;

    for c in source.chars() {
        match c {
            '0'..='9' => {
                counter *= 10;
                counter += c.to_digit(10).unwrap();
            }
            c => {
                if counter == 0 {
                    result.push(c);
                } else {
                    result.extend(std::iter::repeat(c).take(counter as usize));
                };
                counter = 0;
            }
        };
    }

    result
}
