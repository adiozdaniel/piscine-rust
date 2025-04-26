pub fn is_empty(v: &str) -> bool {
    if v.len()==0{
        return true;
    }
    false
}

pub fn is_ascii(v: &str) -> bool {
    v.chars().all(|c| c.is_ascii())
}

pub fn contains(v: &str, pat: &str) -> bool {
    v.contains(pat)
}

pub fn split_at(v: &str, index: usize) -> (&str, &str) {
   v.split_at(index)
}

pub fn find(v: &str, pat: char) -> usize {
  if let Some(i) = v.find(pat){
    return  i;
   }
   return 0;
}

#[cfg(test)]
mod find_tests {
    use super::find;

    #[test]
    fn finds_char() {
        assert_eq!(find("hello", 'e'), 1);
    }

    #[test]
    fn finds_first_occurrence() {
        assert_eq!(find("hello", 'l'), 2);
    }

    #[test]
    fn returns_zero_when_not_found() {
        assert_eq!(find("hello", 'z'), 0);
    }

    #[test]
    fn finds_unicode_char() {
        assert_eq!(find("rust🦀", '🦀'), 4);
    }

    #[test]
    fn empty_string_returns_zero() {
        assert_eq!(find("", 'a'), 0);
    }
}
