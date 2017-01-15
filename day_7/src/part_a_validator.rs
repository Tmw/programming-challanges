use ip_address::{ IPAddress, IPAddressValidator };

pub struct PartAValidator;
impl IPAddressValidator for PartAValidator {
    fn validate(&self, ip_address: &IPAddress) -> bool {

        if ip_address.hypernets.iter().any(contains_abba) {
            return false;
        }

        if ip_address.segments.iter().any(contains_abba) {
            return true;
        }

        false
    }
}

// accepts a whole string and tries to find an ABBA
fn contains_abba(input: &String) -> bool {
    let chars: Vec<String> = input.chars().map(|v| v.to_string()).collect();
    chars.windows(4).any(|s| { is_abba(s.join("").as_str()) })
}

// checks if passed string is an ABBA
fn is_abba(input: &str) -> bool {
    input[0..1] == input[3..4] && input[1..2] == input[2..3] && input[0..1] != input[1..2]
}

#[cfg(test)]
mod is_abba {
    use part_a_validator::is_abba;

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
mod contains_abba {
    use part_a_validator::contains_abba;

    #[test]
    fn detects_abba_containment() {
        assert!(contains_abba(&"abba[arca]zyui".to_string()));
    }

    #[test]
    fn ignores_if_no_abba_present() {
        assert!(!contains_abba(&"abcd[axop]irer".to_string()));
    }
}

#[cfg(test)]
mod validate {

    use ip_address::{ IPAddress, IPAddressValidator };
    use part_a_validator::PartAValidator;

    #[test]
    fn it_should_return_true_for_valid_addresses() {
        let validator = PartAValidator{};
        let subjects : Vec<IPAddress> = vec![
            "abba[zas]errz".parse().unwrap(),
            "noot[zsbr]abba[bluut]".parse().unwrap(),
        ];
        
        for subject in subjects {
            assert_eq!(validator.validate(&subject), true);
        }
    }

    #[test]
    fn it_should_return_false_if_no_abbas_found() {
        let validator = PartAValidator{};
        let subjects : Vec<IPAddress> = vec![
            "no[ab]bas[in]this[string]".parse().unwrap(),
            "nothing[abba]mec".parse().unwrap(),
        ];
        
        for subject in subjects {
            assert_eq!(validator.validate(&subject), false);
        }
    }

    #[test]
    fn it_should_return_false_if_abba_in_hypernet() {
        let validator = PartAValidator{};
        let subjects : Vec<IPAddress> = vec![
            "abba[blaat]more[abba]".parse().unwrap(),
            "acca[abba]should[not]".parse().unwrap(),
        ];
        
        for subject in subjects {
            assert_eq!(validator.validate(&subject), false);
        }
    }

}