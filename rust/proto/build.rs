fn main() {
    tonic_build::compile_protos("../../proto_schema/data.proto").unwrap();
}