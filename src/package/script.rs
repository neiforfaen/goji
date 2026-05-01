#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Script {
    pub name: String,
}

impl Script {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_accepts_str_slice() {
        let s = Script::new("build");
        assert_eq!(s.name, "build");
    }

    #[test]
    fn new_accepts_owned_string() {
        let s = Script::new(String::from("test"));
        assert_eq!(s.name, "test");
    }

    #[test]
    fn equal_when_names_match() {
        assert_eq!(Script::new("build"), Script::new("build"));
    }

    #[test]
    fn not_equal_when_names_differ() {
        assert_ne!(Script::new("build"), Script::new("test"));
    }

    #[test]
    fn clone_produces_identical_script() {
        let a = Script::new("dev");
        assert_eq!(a.clone(), a);
    }

    #[test]
    fn debug_includes_name() {
        assert!(format!("{:?}", Script::new("lint")).contains("lint"));
    }
}
