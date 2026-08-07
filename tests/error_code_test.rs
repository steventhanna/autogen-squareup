//! Guards the ErrorCode post-generation fix in generate.sh.
//!
//! Square's published spec omits some error codes the live API actually
//! returns (e.g. ISSUER_INSTALLMENT_ERROR, which only appears in an
//! x-endpoint-errors vendor extension). The strict generated enum would
//! fail deserialization of the entire API response over one such code.

use autogen_squareup::models::{Error, ErrorCode};

#[test]
fn deserializes_issuer_installment_error() {
    let json = r#"{"category":"PAYMENT_METHOD_ERROR","code":"ISSUER_INSTALLMENT_ERROR"}"#;
    let err: Error = serde_json::from_str(json).expect("known live API code must deserialize");
    assert_eq!(err.code, ErrorCode::IssuerInstallmentError);
}

#[test]
fn unknown_error_code_falls_back_instead_of_failing() {
    let json = r#"{"category":"API_ERROR","code":"SOME_FUTURE_CODE_NOT_IN_SPEC"}"#;
    let err: Error = serde_json::from_str(json).expect("unknown codes must not fail deserialization");
    assert_eq!(err.code, ErrorCode::Unknown);
}

#[test]
fn known_codes_still_roundtrip() {
    let json = r#"{"category":"API_ERROR","code":"RATE_LIMITED"}"#;
    let err: Error = serde_json::from_str(json).unwrap();
    assert_eq!(err.code, ErrorCode::RateLimited);
    assert_eq!(serde_json::to_string(&err).unwrap(), json);
}
