extern crate regex;

use regex::Regex;

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
}
