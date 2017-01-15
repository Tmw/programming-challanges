use std::str::FromStr;
use regex::Regex;

lazy_static!{
    static ref REGEX : Regex = Regex::new(r"\[(?P<hypernet>[a-z]*)\]|(?P<segment>[a-z]*)+").unwrap(); 
}

pub type Segments = Vec<String>;
pub type Hypernets = Vec<String>;

#[derive(Debug)]
pub struct IPAddress {
    pub full_address: String,
    pub segments: Segments,
    pub hypernets: Hypernets,
}

pub trait IPAddressValidator {
    fn validate(&self, subject: &IPAddress) -> bool;
}

impl IPAddress {
    pub fn is_valid<T: IPAddressValidator>(&self, validator: &T) -> bool {
        validator.validate(&self)
    }

    fn dissect_address(s: &str) -> (Segments, Hypernets) {
        let initial: (Segments, Hypernets) = (Vec::new(), Vec::new());
        REGEX.captures_iter(&s).fold(initial, |matches, capture| {
            let mut matches = matches.clone();

            if let Some(segment) = capture.name("segment") {
                matches.0.push(segment.as_str().to_string());
            }

            if let Some(hypernet) = capture.name("hypernet") {
                matches.1.push(hypernet.as_str().to_string());
            }

            matches
        })
    }
}

impl FromStr for IPAddress {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (segments, hypernets) = IPAddress::dissect_address(&s);
        Ok(IPAddress {
            full_address: String::from_str(&s).unwrap(),
            segments: segments,
            hypernets: hypernets,
        })
    }
}

#[cfg(test)]
mod parse_from_str {
    use ip_address::IPAddress;

    #[test]
    fn it_should_parse_from_str() {
        let addr: IPAddress = "some[ip]address[with]brackets".parse().unwrap();
        assert_eq!(addr.full_address, format!("some[ip]address[with]brackets"));
        assert_eq!(addr.hypernets, vec!["ip", "with"]);
        assert_eq!(addr.segments, vec!["some", "address", "brackets"]);
    }
}
