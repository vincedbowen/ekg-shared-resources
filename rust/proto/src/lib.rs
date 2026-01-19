use proto_sign::generate_fingerprint;

// Proto-generated code
pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/ekg.proto.v0.rs"));
}

/// Protobuf Fingerprint
/// 
///  Uses `cargo-sign` to generate a fingerprint ensuring client and server have equivalent fingerprints.
/// 
/// # Returns
/// 
/// String hash with Protobuf fingerprint
pub fn schema_fingerprint_from_file() -> String {
    const PROTO: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../proto_schema/data.proto"));
    generate_fingerprint(PROTO).expect("failed to generate fingerprint")
}
