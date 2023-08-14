pub fn solution(chars: &mut Vec<char>) -> i32 {
    // iteration index
    let mut i = 1;
    // current character index
    let mut cci = 0;
    // count of current character
    let mut count = 1;
    // current character
    let mut c: char = chars[0];
    // println!("chars: {chars:?}");
    while i < chars.len() {
        //     println!("i: {i}, c: {c}, count: {count}, ci: {cci}");
        if chars[i] == c {
            count += 1;
        }
        if chars[i] != c {
            c = chars[i];
            if count > 1 {
                let count_str = format!("{count}");
                chars.splice((cci + 1)..i, count_str.chars());
                i = cci + count_str.len() + 1;
            } else {
                chars.splice((cci + 1)..i, []);
            }
            cci = i;
            count = 1;
        } else if i == chars.len() - 1 {
            if count > 1 {
                chars.splice((cci + 1)..chars.len(), format!("{count}").chars());
            } else {
                chars.splice((cci + 1)..chars.len(), []);
            }
        }
        i += 1;
        //     println!("chars: {chars:?}");
    }
    chars.len().try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::string_compression::solution;

    #[test]
    fn ex_1() {
        let mut chars = vec!['a', 'a', 'b', 'b', 'c', 'c', 'c'];
        assert_eq!(solution(&mut chars), 6);
        assert_eq!(chars, ['a', '2', 'b', '2', 'c', '3']);
    }

    #[test]
    fn ex_2() {
        let mut chars = vec!['a'];
        assert_eq!(solution(&mut chars), 1);
        assert_eq!(chars, ['a']);
    }

    #[test]
    fn ex_3() {
        let mut chars = vec![
            'a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b',
        ];
        assert_eq!(solution(&mut chars), 4);
        assert_eq!(chars, ['a', 'b', '1', '2']);
    }

    #[test]
    fn ex_4() {
        let mut chars = vec!['a', 'a', 'a', 'b', 'b', 'a', 'a'];
        assert_eq!(solution(&mut chars), 6);
        assert_eq!(chars, ['a', '3', 'b', '2', 'a', '2']);
    }
}
