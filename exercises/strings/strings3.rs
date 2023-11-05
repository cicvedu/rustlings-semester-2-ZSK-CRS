// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a
// hint.


fn trim_me(input: &str) -> &str {
    input.trim()
}

fn compose_me(mut input: String) -> String {
    // TODO: Add " world!" to the string! There's multiple ways to do this!
    input.push_str(" world!");
    return input;

    // format!("{} world!", input) // 审题
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons"!
    //replace是返回一个新的字符串，而不是操作原来的字符串,所以不用不用管是String还是切片
    input.replace("cars", "balloons") // 替换单词
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello".to_string()), "Hello world!");
        assert_eq!(compose_me("Goodbye".to_string()), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
