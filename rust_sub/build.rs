fn main() {
    prost_build::compile_protos(
        &["../protos/hello.proto", "../protos/metrics.proto"],
        &["../protos"],
    ).expect("failed to compile protos");
}
