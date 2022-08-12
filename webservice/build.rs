fn main() {
    // std::env::set_var("PROTOC", protobuf_src::protoc());
    // let proto_file = "./proto/hello.proto"; 

    // tonic_build::configure()
    //     .build_server(true)
    //     .compile(&[proto_file], &["."])
    //     .unwrap_or_else(|e| panic!("protobuf compile error: {}", e));
  
    //     println!("cargo:rerun-if-changed={}", proto_file);
    let proto_file = "./grpc_proto/Plat/AccountService.proto"; 

    tonic_build::configure()
        .build_server(true)
        .compile(&[proto_file], &["."])
        .unwrap_or_else(|e| panic!("protobuf compile error: {}", e));
  
        println!("cargo:rerun-if-changed={}", proto_file);
}