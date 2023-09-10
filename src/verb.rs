#[derive(Debug, Clone, PartialEq)]
pub struct Verb {
    pub verb_type: VerbType,
    pub form: VerbInflection,
}

#[derive(Debug, Clone, PartialEq)]
pub enum VerbType {
    Ichidan,
    Godan,
    Suru,
    Kuru,
    Exception,
}

#[derive(Debug, Clone, PartialEq)]
pub enum VerbInflection {
    Dictionary,
    Polite,
}

impl VerbInflection {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "" => None,
            _ => None,
        }
    }
}
