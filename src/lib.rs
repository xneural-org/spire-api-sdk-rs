pub mod spire {
    pub mod api {
        pub mod types {
            tonic::include_proto!("spire.api.types");
        }
        pub mod server {
            pub mod entry {
                pub mod v1 {
                    tonic::include_proto!("spire.api.server.entry.v1");
                }
            }
            pub mod agent {
                pub mod v1{
                    tonic::include_proto!("spire.api.server.agent.v1");
                }
            }
        }
    }
}
