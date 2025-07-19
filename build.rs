
fn main() {
    tonic_build::compile_protos("balanderapi/service.proto").unwrap();
    println!("cargo:rerun-if-changed=balancerapi/service.proto");
}
