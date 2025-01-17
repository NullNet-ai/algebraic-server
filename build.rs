const ALGEBRAIC_PROTOBUF_PATH: &str = "./proto/algebraic.proto";
const PROTOBUF_DIR_PATH: &str = "./proto";

fn main() {
    tonic_build::configure()
        .out_dir("./src/proto")
        .compile_protos(&[ALGEBRAIC_PROTOBUF_PATH], &[PROTOBUF_DIR_PATH])
        .expect("Protobuf files generation failed");
}
