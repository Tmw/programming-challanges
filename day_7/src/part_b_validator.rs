use ip_address::{ IPAddress, IPAddressValidator };

pub struct PartBValidator;
impl IPAddressValidator for PartBValidator {
    fn validate(&self, ip_address: &IPAddress) -> bool {

        // convert segments to ABAs
        let abas : Vec<String> = ip_address.segments.iter().flat_map(|s| {
            let chars: Vec<String> = s.chars().map(|c| c.to_string()).collect();
            let abas: Vec<String> = chars.windows(3)
                .map(|s| s.join(""))
                .filter(|s| { is_aba(s.as_str()) })
                .collect();

            abas
        }).collect();

        // early return if no ABAs are found
        if abas.is_empty() {
            return false;
        }

        // convert ABAs to BABs
        let babs: Vec<String> = abas.iter().map(|s| get_bab(s.as_str())).collect();

        // does BAB exist in hypernets?
        babs.iter().any(|b|{
            ip_address.hypernets.iter().any(|h| {
                h.contains(b)
            })
        })
    }
}

// check if three letter sequence if ABA
fn is_aba(s: &str) -> bool {
    s[0..1] == s[2..3] && s[0..1] != s[1..2]
}

// accept a ABA and generate its BAB
fn get_bab(aba: &str) -> String {
    let bab: [&str;3] = [&aba[1..2], &aba[0..1], &aba[1..2]];
    bab.join("")
}

#[cfg(test)]
mod is_aba {
    use part_b_validator::is_aba;

    #[test]
    fn it_should_return_true_if_is_aba() {
        assert!(is_aba("mem"));
        assert!(is_aba("ioi"));
    }

    #[test]
    fn it_should_return_false_if_invalid() {
        assert!(!is_aba("ooo"));
        assert!(!is_aba("qqq"));
    }
}

#[cfg(test)]
mod get_bab {
    use part_b_validator::get_bab;

    #[test]
    fn it_should_generate_bab_for_aba() {
        assert_eq!(get_bab("bab"), "aba");
    }
}

#[cfg(test)]
mod validate {
    use part_b_validator::PartBValidator;
    use ip_address::{ IPAddress, IPAddressValidator };

    #[test]
    fn it_should_return_true_for_valids() {
        let validator = PartBValidator{};
        let subjects : Vec<IPAddress> = vec![
            "aba[bab]".parse().unwrap(),
            "eraoporel[iuasi]iruielp[pop]ruerj".parse().unwrap(),
        ];

        for subject in subjects {
            assert!(validator.validate(&subject));
        }
    }

    #[test]
    fn it_should_return_false_for_invalids() {
        let validator = PartBValidator{};
        let subjects : Vec<IPAddress> = vec![
            "abs[bab]".parse().unwrap(),
            "eraruyrel[iuasi]iruielp[pop]ruerj".parse().unwrap(),
        ];

        for subject in subjects {
            assert!(!validator.validate(&subject));
        }
    }

}