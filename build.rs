
fn main() {
    tonic_build::compile_protos("balancerapi/service.proto").unwrap();
}
