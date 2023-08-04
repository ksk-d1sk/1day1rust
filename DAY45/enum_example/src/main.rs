use std::cmp::Ordering::{self, *};
use std::mem::size_of;

enum HttpStatus {
    Ok = 200,
    NotModified = 304,
    NotFound = 404,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TimeUnit {
    Seconds, Minutes, Hours, Days, Months, Years,
}

impl TimeUnit {
    fn plural(self) -> &'static str {
        match self {
            TimeUnit::Seconds => "seconds",
            TimeUnit::Minutes => "minutes",
            TimeUnit::Hours => "hours",
            TimeUnit::Days => "days",
            TimeUnit::Months => "months",
            TimeUnit::Years => "years",
        }
    }

    fn singular(self) -> &'static str {
        self.plural().trim_end_matches('s')
    }
}

fn compare(n: i32, m: i32) -> Ordering {
    if n < m {
        Less
    } else if n == m{
        Greater
    } else {
        Equal
    }
}

fn http_status(n: u32) -> Option<HttpStatus> {
    match n {
        200 => Some(HttpStatus::Ok),
        304 => Some(HttpStatus::NotModified),
        404 => Some(HttpStatus::NotFound),
        _   => None,
    }
}

fn main() {
    assert_eq!(size_of::<Ordering>(), 1);
    assert_eq!(size_of::<HttpStatus>(), 2);
}
