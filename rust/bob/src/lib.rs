pub fn reply(message: &str) -> &str {
    const SURE: &str = "Sure.";
    const CHILL: &str = "Whoa, chill out!";
    const CALM: &str = "Calm down, I know what I'm doing!";
    const FINE: &str = "Fine. Be that way!";
    const WHATEVER: &str = "Whatever.";

    let message = message.trim();

    if message.is_empty() {
        return FINE;
    }

    let is_question = message.chars().last() == Some('?');
    let is_yell =
        message.contains(|c| 'A' <= c && c <= 'Z') && !message.contains(|c| 'a' <= c && c <= 'z');

    if is_question {
        if is_yell {
            return CALM;
        }
        return SURE;
    }

    if is_yell {
        return CHILL;
    }

    WHATEVER
}
