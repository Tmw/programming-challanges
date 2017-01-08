/*
--- Day 7: Internet Protocol Version 7 ---

While snooping around the local network of EBHQ, you compile a list of IP addresses (they're IPv7, of course; IPv6 is much too limited). You'd like to figure out which IPs support TLS (transport-layer snooping).

An IP supports TLS if it has an Autonomous Bridge Bypass Annotation, or ABBA. An ABBA is any four-character sequence which consists of a pair of two different characters followed by the reverse of that pair, such as xyyx or abba. However, the IP also must not have an ABBA within any hypernet sequences, which are contained by square brackets.

For example:

abba[mnop]qrst supports TLS (abba outside square brackets).
abcd[bddb]xyyx does not support TLS (bddb is within square brackets, even though xyyx is outside square brackets).
aaaa[qwer]tyui does not support TLS (aaaa is invalid; the interior characters must be different).
ioxxoj[asdfgh]zxcvbn supports TLS (oxxo is outside square brackets, even though it's within a larger string).
How many IPs in your puzzle input support TLS?

*/

extern crate regex;
#[macro_use] extern crate lazy_static;

use regex::Regex;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    // read input from file
    let mut f = File::open("input.txt").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).ok();

    // convert input to vec and filter valids
    let input_as_vec : Vec<String> = input.lines().into_iter().map(|l| l.to_string()).collect();
    let answer = filter_valids(&input_as_vec);

    // hooray
    println!("[PART A] answer: {}", answer.len());
}

fn filter_valids(input : &Vec<String>) -> Vec<String> {
    input.iter().cloned().filter(|s| contains_valid_abba(s)).collect()
}

fn contains_valid_abba(input : &String) -> bool {
    let abbas = get_abbas(input);
    if abbas.is_empty() { 
        return false;
    }

    lazy_static! {
        // regex grabbing the [ and ] pairs
        static ref REGEX : Regex = Regex::new(r"\[([a-z]*)\]+").unwrap();
    }

    // if we find an abba in one of the pairs, early return
    for capture in REGEX.captures_iter(input) {
        if has_abba(&capture[1].to_string()) {
            return false;
        }
    }

    true
}

fn has_abba(input : &String) -> bool {
    !get_abbas(input).is_empty()
}

fn get_abbas(input : &String) -> Vec<String> {
    let chars : Vec<String> = input.chars().map(|v| v.to_string()).collect();
    chars.windows(4).map(|v| v.concat()).filter(|v| is_abba(&v)).collect()
}

fn is_abba(input : &str) -> bool {
    input[0..1] == input[3..4] && 
    input[1..2] == input[2..3] &&
    input[0..1] != input[1..2]
}

#[cfg(test)]
mod is_abba {
    use ::is_abba;

    #[test]
    fn should_detect_valid_abbas() {
        assert!(is_abba("abba"));
    }

    #[test]
    fn should_ignore_invalid_sequences() {
        assert!(!is_abba("azop"));
    }

    #[test]
    fn should_ignore_if_four_characters_match() {
        assert!(!is_abba("aaaa"));
    }
}

#[cfg(test)]
mod get_abbas {
    use ::get_abbas;

    #[test]
    fn returns_empty_vec_if_no_abbas() {
        assert!(get_abbas(&"AZXD[ORPF]UPOP]".to_string()).is_empty());
    }

    #[test]
    fn returns_abbas_in_order() {
        let abbas = get_abbas(&"ABBA[AZBC]ARRA".to_string());
        assert_eq!(abbas, vec!["ABBA".to_string(), "ARRA".to_string()]);
    }
}

#[cfg(test)]
mod contains_abba {
    use ::contains_valid_abba;

    #[test]
    fn detects_abba_containment() {
        assert!(contains_valid_abba(&"abba[arca]zyui".to_string()));
    }

    #[test]
    fn ignores_if_no_abba_present() {
        assert!(!contains_valid_abba(&"abcd[axop]irer".to_string()));
    }

    #[test]
    fn ignores_if_abba_is_in_brackets() {
        assert!(!contains_valid_abba(&"abcd[abba]irer".to_string()));
    }

    #[test]
    fn ignores_if_another_abba_is_within_brackets() {
        assert!(!contains_valid_abba(&"abba[axxa]irer".to_string()));
    }
}

#[cfg(test)]
mod filter_valids {
    use ::filter_valids;

    #[test]
    fn should_pass_for_examples() {
        let examples = vec![
            "abba[mnop]qrst".to_string(),
            "abcd[bddb]xyyx".to_string(),
            "aaaa[qwer]tyui".to_string(),
            "ioxxoj[asdfgh]zxcvbn".to_string(),
        ];

        let valids = filter_valids(&examples);
        assert_eq!(valids.len(), 2);
    }

    #[test]
    fn should_filter_out_invalids() {
        let examples = vec![
            "heuj[yayy]mecmec".to_string(),
            "eroa[eiro]aopp".to_string(),
            "yurppa[rabbao]oprz".to_string(),
            "uiabbaii[aopo]arza".to_string(),
            "azabbaeeae[ahuuaal]eorpoa[arrapekl]".to_string(),
        ];

        let valids = filter_valids(&examples);
        assert_eq!(valids.len(), 1);
    }
}

#[cfg(test)]
mod has_abba {
    use ::has_abba;

    #[test]
    fn returns_true_if_given_string_contains_abba() {
        assert!(has_abba(&"aopabbaeuia".to_string()));
    }

    #[test]
    fn returns_false_if_given_string_doesnt_contain_abba(){
        assert!(!has_abba(&"aioierlddfuiw".to_string()));
    }
}
