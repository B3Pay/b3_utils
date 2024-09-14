//! Domain separation context types for hashing.
use std::convert::TryFrom;
use std::fmt;

/// A domain separation context that can be represented as bytes.
pub trait Context {
    fn as_bytes(&self) -> &[u8];
}

/// A domain separation context based on a `String` to separate domains when
/// hashing.
pub struct DomainSeparationContext {
    domain: String,
    bytes: Vec<u8>,
}

impl DomainSeparationContext {
    /// Returns a new domain separation context.
    ///
    /// The byte representation of the context is a concatenation of
    /// * one byte indicating the length of the domain's UTF-8 bytes
    /// * the domain's UTF-8 bytes
    ///
    /// Panics if the length of the domain's UTF-8 bytes is too long to fit into
    /// one byte (that is, greater than 255).
    pub fn new<S: Into<String>>(domain: S) -> DomainSeparationContext {
        let domain: String = domain.into();

        let domain_bytes: &[u8] = domain.as_bytes();
        let domain_bytes_len = u8::try_from(domain_bytes.len()).expect("domain too long");

        let mut bytes = vec![domain_bytes_len];
        bytes.extend(domain_bytes);

        DomainSeparationContext { domain, bytes }
    }

    /// Returns the domain as string.
    #[allow(dead_code)]
    pub fn domain(&self) -> &String {
        &self.domain
    }
}

impl fmt::Debug for DomainSeparationContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "DomainSeparationContext{{ domain: \"{}\" }}",
            &self.domain
        )
    }
}

impl Context for DomainSeparationContext {
    fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }
}

#[test]
fn should_return_correct_byte_representation() {
    let context = DomainSeparationContext::new("test-🦀");
    assert_eq!(
        context.as_bytes(),
        [9, b't', b'e', b's', b't', b'-', 240, 159, 166, 128]
    );
}

#[test]
fn should_return_correct_byte_representation_for_empty_domain() {
    let context = DomainSeparationContext::new("");
    assert_eq!(context.as_bytes(), [0]);
}

#[test]
fn should_return_correct_domain() {
    let context = DomainSeparationContext::new("test-🦀");
    assert_eq!(context.domain(), "test-🦀");
}

#[test]
fn should_return_correct_empty_domain() {
    let context = DomainSeparationContext::new("");
    assert_eq!(context.domain(), "");
}

#[test]
#[should_panic(expected = "domain too long")]
fn should_panic_if_domain_too_long_for_1_byte_length_prefix() {
    let _panic = DomainSeparationContext::new("a".repeat(256));
}

#[test]
fn should_be_instantiable_for_domains_with_maximum_possible_length() {
    let _ = DomainSeparationContext::new("a".repeat(255));
}

#[test]
fn should_be_instantiable_from_string() {
    let context = DomainSeparationContext::new(String::from("test"));
    assert_eq!(context.domain, "test");
}

#[test]
fn should_be_instantiable_from_str() {
    let context = DomainSeparationContext::new("test");
    assert_eq!(context.domain, "test");
}

#[test]
fn should_debug_pretty_print_domain_separation_context() {
    let test_vectors = vec![(
        DomainSeparationContext::new("test"),
        "DomainSeparationContext{ domain: \"test\" }",
    )];
    for (value, formatted) in test_vectors {
        assert_eq!(format!("{:?}", value), *formatted);
    }
}
