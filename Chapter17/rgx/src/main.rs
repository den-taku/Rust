extern crate regex;
extern crate unicode_normalization;

#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::io::BufRead;
use unicode_normalization::UnicodeNormalization;

lazy_static! {
    static ref SEMVER: Regex
        = Regex::new(r"(\d+)\.(\d+)\.(\d+)(-[-.[:alnum:]])?")
            .expect("error parsing regex");
}

fn main() {
    // A semver version number, like 0.2.1.
    // May contain a pre-release version suffix, like 0.2.1-alpha.
    // (No build metadata suffix, for brevity.)
    //
    // Note use of r"..." raw string syntax, to avoid backslash blizzard.
    let semver = Regex::new(r"(\d+)\.(\d+)\.(\d+)(-[-.[:alnum:]]*)?").expect("err");

    // Simple search, with a Boolean result.
    let haystack = r#"regex = "0.2.5""#;
    if semver.is_match(haystack) {
        println!("{} matched", haystack);
    }

    // You can retrieve capture groups:
    let captures = semver.captures(haystack)
        .ok_or("semver regex should have matched").expect("err");
    println!("{:#?}", captures);
    println!("{:?}", captures.get(4));
    println!("{:?}", captures.get(3).unwrap().start());
    println!("{:?}", captures.get(3).unwrap().end());
    println!("{:?}", captures.get(3).unwrap().as_str());

    let haystack = "In the beginning, there was 1.0.0. \
                    For a while. we used 1.0.1-beta, \
                    but in the end, we settled on 1.2.4.";
    let matches: Vec<&str> = semver.find_iter(haystack)
        .map(|match_| match_.as_str())
        .collect();
    println!("{:?}", matches);

    let captures: Vec<_> = semver.captures_iter(haystack)
        .collect();
    println!("{:#?}", captures);

    // let stdin = std::io::stdin();
    // for line in  stdin.lock().lines() {
    //     let line = line.expect("err");
    //     if let Some(match_) = SEMVER.find(&line) {
    //         println!("{}", match_.as_str());
    //     }
    // }

    assert!("th\u{e9}" != "the\u{301}");

    assert_eq!("① Di\u{fb03}culty".nfkc().collect::<String>(), "1 Difficulty");
}
