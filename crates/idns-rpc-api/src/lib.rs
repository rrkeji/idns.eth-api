pub mod grpc {
    tonic::include_proto!("idns.rpc");

    pub const FILE_DESCRIPTOR_SET: &'static [u8] =
        tonic::include_file_descriptor_set!("idns_rpc_descriptor");
}
