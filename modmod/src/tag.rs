use std::fmt::Display;

pub trait ToTag: ToString {
    fn to_prefixed_tag<P: Display>(&self, prefix: P) -> String {
        format!("{prefix}-{s}", s = self.to_string()).to_tag()
    }

    fn to_tag(&self) -> String;
}

impl<T: ToString> ToTag for T {
    fn to_tag(&self) -> String {
        let mut s = self.to_string();
        s.make_ascii_lowercase();
        let mut tag = String::new();
        let mut words = s.split_whitespace();

        match words.next() {
            Some(w) => tag.push_str(w),
            None => return s,
        }

        for word in words {
            tag.push('-');
            tag.push_str(word);
        }
        tag
    }
}
