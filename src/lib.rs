//! A crate providing access to the [`NAME`] "jordin".
//! 
//! # Example
//! ```rust
//! // Print "jordin" to stdout, without a newline
//! jordin::print_name(); // "jordin" has been printed to stdout
//! 
//! // Print "jordin" to stdout, with a newline
//! jordin::println_name(); // "jordin" has been printed to stdout
//! 
//! // NAME shall be equal to "jordin"
//! assert_eq!(jordin::NAME, "jordin");
//! // NAME shall not be equal to "Jordin" (see NAME_ADHERING_TO_ENGLISH_PROPER_NOUN_CAPITALISATION_RULES).
//! assert_ne!(jordin::NAME, "Jordin");
//! 
//! // NAME shall not be confused with the following similar but incorrect names
//! assert_ne!(jordin::NAME, "jordan");
//! assert_ne!(jordin::NAME, "jorden");
//! assert_ne!(jordin::NAME, "jordon");
//! assert_ne!(jordin::NAME, "jordun");
//! assert_ne!(jordin::NAME, "jordyn");
//! 
//! // NAME_ADHERING_TO_ENGLISH_PROPER_NOUN_CAPITALISATION_RULES shall be equal to "Jordin"
//! assert_eq!(jordin::NAME_ADHERING_TO_ENGLISH_PROPER_NOUN_CAPITALISATION_RULES, "Jordin");
//! // NAME_ADHERING_TO_ENGLISH_PROPER_NOUN_CAPITALISATION_RULES shall not be equal to "jordin" (see NAME) 
//! assert_ne!(jordin::NAME_ADHERING_TO_ENGLISH_PROPER_NOUN_CAPITALISATION_RULES, "jordin");
//! 
//! // NAME_ADHERING_TO_ENGLISH_PROPER_NOUN_CAPITALISATION_RULES shall not be confused with the following similar but incorrect names
//! assert_ne!(jordin::NAME_ADHERING_TO_ENGLISH_PROPER_NOUN_CAPITALISATION_RULES, "Jordan");
//! assert_ne!(jordin::NAME_ADHERING_TO_ENGLISH_PROPER_NOUN_CAPITALISATION_RULES, "Jorden");
//! assert_ne!(jordin::NAME_ADHERING_TO_ENGLISH_PROPER_NOUN_CAPITALISATION_RULES, "Jordon");
//! assert_ne!(jordin::NAME_ADHERING_TO_ENGLISH_PROPER_NOUN_CAPITALISATION_RULES, "Jordun");
//! assert_ne!(jordin::NAME_ADHERING_TO_ENGLISH_PROPER_NOUN_CAPITALISATION_RULES, "Jordyn");
//! ```

/// The name "jordin".
/// 
/// # Example
/// ```rust
/// // NAME shall be equal to "jordin"
/// assert_eq!(jordin::NAME, "jordin");
/// // NAME shall not be equal to "Jordin" (see NAME_ADHERING_TO_ENGLISH_PROPER_NOUN_CAPITALISATION_RULES).
/// assert_ne!(jordin::NAME, "Jordin");
/// 
/// // NAME shall not be confused with the following similar but incorrect names
/// assert_ne!(jordin::NAME, "jordan");
/// assert_ne!(jordin::NAME, "jorden");
/// assert_ne!(jordin::NAME, "jordon");
/// assert_ne!(jordin::NAME, "jordun");
/// assert_ne!(jordin::NAME, "jordyn");
/// ```
pub const NAME: &str = "jordin";

/// The name "Jordin", which adheres to the English language proper noun capitalisation rules.
/// 
/// # Example
/// ```rust
/// // NAME_ADHERING_TO_ENGLISH_PROPER_NOUN_CAPITALISATION_RULES shall be equal to "Jordin"
/// assert_eq!(jordin::NAME_ADHERING_TO_ENGLISH_PROPER_NOUN_CAPITALISATION_RULES, "Jordin");
/// // NAME_ADHERING_TO_ENGLISH_PROPER_NOUN_CAPITALISATION_RULES shall not be equal to "jordin" (see NAME) 
/// assert_ne!(jordin::NAME_ADHERING_TO_ENGLISH_PROPER_NOUN_CAPITALISATION_RULES, "jordin");
/// 
/// // NAME_ADHERING_TO_ENGLISH_PROPER_NOUN_CAPITALISATION_RULES shall not be confused with the following similar but incorrect names
/// assert_ne!(jordin::NAME_ADHERING_TO_ENGLISH_PROPER_NOUN_CAPITALISATION_RULES, "Jordan");
/// assert_ne!(jordin::NAME_ADHERING_TO_ENGLISH_PROPER_NOUN_CAPITALISATION_RULES, "Jorden");
/// assert_ne!(jordin::NAME_ADHERING_TO_ENGLISH_PROPER_NOUN_CAPITALISATION_RULES, "Jordon");
/// assert_ne!(jordin::NAME_ADHERING_TO_ENGLISH_PROPER_NOUN_CAPITALISATION_RULES, "Jordun");
/// assert_ne!(jordin::NAME_ADHERING_TO_ENGLISH_PROPER_NOUN_CAPITALISATION_RULES, "Jordyn");
/// ```
pub const NAME_ADHERING_TO_ENGLISH_PROPER_NOUN_CAPITALISATION_RULES: &str = "Jordin";

/// Print the [`NAME`] ("jordin") to the standard output of the current process, without a newline.
/// 
/// # Example
/// ```rust
/// // Print "jordin" to stdout, without a newline
/// jordin::print_name(); // "jordin" has been printed to stdout
/// ```
pub fn print_name() {
    print!("{}", NAME);
}

/// Print the [`NAME`] ("jordin") to the standard output of the current process, with a newline.
/// 
/// # Example
/// ```rust
/// // Print "jordin" to stdout, with a newline
/// jordin::println_name(); // "jordin" has been printed to stdout
/// ```
pub fn println_name() {
    println!("{}", NAME);
}

/// Print the [`NAME_ADHERING_TO_ENGLISH_PROPER_NOUN_CAPITALISATION_RULES`] ("Jordin") to the standard output of the current process, without a newline.
/// 
/// # Example
/// ```rust
/// // Print "Jordin" to stdout, without a newline
/// jordin::print_name(); // "Jordin" has been printed to stdout
/// ```
pub fn print_name_adhering_to_english_proper_noun_capitalisation_rules() {
    print!("{}", NAME_ADHERING_TO_ENGLISH_PROPER_NOUN_CAPITALISATION_RULES);
}

/// Print the [`NAME_ADHERING_TO_ENGLISH_PROPER_NOUN_CAPITALISATION_RULES`] ("Jordin") to the standard output of the current process, with a newline.
/// 
/// # Example
/// ```rust
/// // Print "Jordin" to stdout, with a newline
/// jordin::println_name(); // "Jordin" has been printed to stdout
/// ```
pub fn println_name_adhering_to_english_proper_noun_capitalisation_rules() {
    println!("{}", NAME_ADHERING_TO_ENGLISH_PROPER_NOUN_CAPITALISATION_RULES);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        assert_eq!(NAME, "jordin");
    }

    #[test]
    fn test_name_adhering_to_english_proper_noun_capitalisation_rules() {
        assert_eq!(NAME_ADHERING_TO_ENGLISH_PROPER_NOUN_CAPITALISATION_RULES, "Jordin");
    }

    #[test]
    fn test_name_confusion() {
        assert_ne!(NAME, "jordan");
        assert_ne!(NAME, "jorden");
        assert_ne!(NAME, "jordon");
        assert_ne!(NAME, "jordun");
        assert_ne!(NAME, "jordyn");
    }

    #[test]
    fn test_name_adhering_to_english_proper_noun_capitalisation_rules_confusion() {
        assert_ne!(NAME_ADHERING_TO_ENGLISH_PROPER_NOUN_CAPITALISATION_RULES, "Jordan");
        assert_ne!(NAME_ADHERING_TO_ENGLISH_PROPER_NOUN_CAPITALISATION_RULES, "Jorden");
        assert_ne!(NAME_ADHERING_TO_ENGLISH_PROPER_NOUN_CAPITALISATION_RULES, "Jordon");
        assert_ne!(NAME_ADHERING_TO_ENGLISH_PROPER_NOUN_CAPITALISATION_RULES, "Jordun");
        assert_ne!(NAME_ADHERING_TO_ENGLISH_PROPER_NOUN_CAPITALISATION_RULES, "Jordyn");
    }
}
