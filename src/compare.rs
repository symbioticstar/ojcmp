use crate::chars::{CharsLike, EOF};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Comparison {
    AC = 100,
    WA = 101,
    PE = 102,
}

/// Compare `std` and `user`. The process will be terminated on error.
pub fn compare(std: &mut impl CharsLike, user: &mut impl CharsLike) -> Comparison {
    let mut stdchar = std.next_char();
    let mut userchar = user.next_char();

    let mut ans = Comparison::AC;

    loop {
        if stdchar == EOF {
            if userchar == EOF {
                return ans;
            }
            if !(userchar as u8).is_ascii_whitespace() {
                return Comparison::WA;
            }
            if poll_eof(user) {
                return ans;
            } else {
                return Comparison::WA;
            }
        }

        if userchar == EOF {
            if stdchar == EOF {
                return ans;
            }
            if !(stdchar as u8).is_ascii_whitespace() {
                return Comparison::WA;
            }
            if poll_eof(std) {
                return ans;
            } else {
                return Comparison::WA;
            }
        }

        let (a, b) = (stdchar as u8, userchar as u8);
        if a == b {
            stdchar = std.next_char();
            userchar = user.next_char();
            continue;
        }

        if a == b'\n' {
            if !b.is_ascii_whitespace() {
                return Comparison::WA;
            }
            if poll_endline(user) {
                stdchar = std.next_char();
                userchar = user.next_char();
                continue;
            } else {
                return Comparison::WA;
            }
        }
        if b == b'\n' {
            if !a.is_ascii_whitespace() {
                return Comparison::WA;
            }
            if poll_endline(std) {
                stdchar = std.next_char();
                userchar = user.next_char();
                continue;
            } else {
                return Comparison::WA;
            }
        }

        let flaga = a.is_ascii_whitespace();
        let flagb = b.is_ascii_whitespace();

        // a != b
        // both of them are non-space
        if !flaga & !flagb {
            return Comparison::WA;
        }

        // a != b
        // both of them are not non-space
        if flaga {
            stdchar = poll_nonspace(std);
        }
        if flagb {
            userchar = poll_nonspace(user);
        }

        if stdchar == EOF || userchar == EOF {
            continue;
        }

        let (a, b) = (stdchar as u8, userchar as u8);
        let flaga = a == b'\n';
        let flagb = b == b'\n';

        if flaga & flagb {
            stdchar = std.next_char();
            userchar = user.next_char();
            continue;
        }
        if flaga | flagb {
            return Comparison::WA;
        }
        if a == b {
            ans = Comparison::PE;
            stdchar = std.next_char();
            userchar = user.next_char();
            continue;
        } else {
            return Comparison::WA;
        }
    }
}

/// poll until eof.
/// ensure that all chars remaining in `chars` are ascii whitespaces
fn poll_eof(chars: &mut impl CharsLike) -> bool {
    let mut c = chars.next_char();
    loop {
        if c == EOF {
            return true;
        }
        if !(c as u8).is_ascii_whitespace() {
            return false;
        }
        c = chars.next_char();
    }
}

/// poll until b'\n'.
/// ensure that all chars remaining in `chars` line are ascii whitespaces
fn poll_endline(chars: &mut impl CharsLike) -> bool {
    let mut c = chars.next_char();
    loop {
        if c == EOF || (c as u8) == b'\n' {
            return true;
        }
        if !(c as u8).is_ascii_whitespace() {
            return false;
        }
        c = chars.next_char();
    }
}

/// poll until b'\n' or non-space or EOF
fn poll_nonspace(chars: &mut impl CharsLike) -> u16 {
    let mut c = chars.next_char();
    loop {
        if c == EOF || (c as u8) == b'\n' || !(c as u8).is_ascii_whitespace() {
            return c;
        }
        c = chars.next_char();
    }
}
