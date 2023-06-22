fn delete_by_pair(input: String) -> String {
    let (mut last_ch, mut last_ch_idx) = (None, 0);
    for (i, ch) in input.chars().enumerate() {
        if last_ch.is_none() {
            last_ch = Some(ch);
            last_ch_idx = i;
            continue;
        }
        if last_ch.unwrap() == ch {
            let input_without_pair = format!("{}{}", &input[..last_ch_idx], &input[i + 1..]);
            return delete_by_pair(input_without_pair);
        }
        (last_ch, last_ch_idx) = (Some(ch), i);
    }
    input
}

#[cfg(test)]
mod tests {
    use crate::delete_by_pair;

    #[test]
    fn basic_test() {
        assert_eq!(delete_by_pair("baaabbacddc".to_string()), "b".to_string());
    }
}
