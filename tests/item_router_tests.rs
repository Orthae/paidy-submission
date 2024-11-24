mod item_router_tests {
    use paidy_submission::application::item_service::MockItemService;
    use paidy_submission::web::item_endpoint::ItemRouter;
    use reqwest::Client;
    use std::net::SocketAddr;
    use std::sync::Arc;
    use tokio::net::TcpListener;

    struct ItemRouterTestContext {
        client: Client,
        address: SocketAddr,
    }

    impl ItemRouterTestContext {
        async fn new(service: MockItemService) -> Self {
            let listener = TcpListener::bind("0.0.0.0:0")
                .await
                .expect("Failed to bind port");

            let address = listener.local_addr()
                .expect("Failed to get local address");

            let router = ItemRouter::new(Arc::new(service));

            tokio::spawn(async move { axum::serve(listener, router).await.expect("Error"); });

            let client = Client::new();

            Self { address, client }
        }
    }
    
    mod item_router_tests {
        const RAW_NOT_FOUND: &str = r#"{"message":"Resource not found"}"#;
        const RAW_PATH_PARSE_ERROR: &str = r#"{"message":"Failed to extract the path parameter."}"#;
        
        mod get_item_endpoint {
            use super::*;
            use crate::item_router_tests::ItemRouterTestContext;
            use chrono::DateTime;
            use mockall::predicate::eq;
            use paidy_submission::application::item_service::{ApplicationError, ItemModel, MockItemService};
            use std::str::FromStr;
            use uuid::Uuid;

            const RAW_ITEM: &str = r#"{"id":"01935dfe-97cf-73b2-be4c-15b3acfc607e","table_id":1,"name":"Pierogi","preparation_time":"2024-11-24T00:00:00Z"}"#;
            
            #[tokio::test]
            async fn should_get_item() {
                let model = ItemModel {
                    id: Uuid::parse_str("01935dfe-97cf-73b2-be4c-15b3acfc607e")
                        .expect("Failed to parse UUID"),
                    table_id: 1,
                    name: "Pierogi".to_string(),
                    preparation_time: DateTime::from_str("2024-11-24T00:00:00Z")
                        .expect("Failed to parse date")
                };

                let mut service = MockItemService::new();
                service.expect_get_item()
                    .with(eq(model.table_id), eq(model.id))
                    .return_const(Ok(model.clone()))
                    .once();

                let context = ItemRouterTestContext::new(service).await;

                let url = format!("http://{}/tables/{}/items/{}", context.address, model.table_id, model.id);
                let response = context.client.get(url)
                    .send()
                    .await
                    .expect("Failed to get response");

                let status = response.status();

                let body = response
                    .text()
                    .await
                    .expect("Failed to get body");

                assert_eq!(status, 200);
                assert_eq!(body, RAW_ITEM);
            }

            #[tokio::test]
            async fn should_return_resource_not_found() {
                let item_id = Uuid::now_v7();
                let table_id = 1;

                let mut service = MockItemService::new();
                service.expect_get_item()
                    .return_const(Err(ApplicationError::ResourceNotFound))
                    .once();

                let context = ItemRouterTestContext::new(service).await;

                let url = format!("http://{}/tables/{}/items/{}", context.address, table_id, item_id);
                let response = context.client.get(url)
                    .send()
                    .await
                    .expect("Failed to get response");

                let status = response.status();

                let body = response
                    .text()
                    .await
                    .expect("Failed to get body");

                assert_eq!(status, 404);
                assert_eq!(body, RAW_NOT_FOUND);
            }

            #[tokio::test]
            async fn should_reject_request_bad_table_id() {
                let item_id = Uuid::now_v7();
                let table_id = "some_table";

                let mut service = MockItemService::new();
                service.expect_get_item()
                    .never();

                let context = ItemRouterTestContext::new(service).await;

                let url = format!("http://{}/tables/{}/items/{}", context.address, table_id, item_id);
                let response = context.client.get(url)
                    .send()
                    .await
                    .expect("Failed to get response");

                let status = response.status();

                let body = response
                    .text()
                    .await
                    .expect("Failed to get body");

                assert_eq!(status, 422);
                assert_eq!(body, RAW_PATH_PARSE_ERROR);
            }

            #[tokio::test]
            async fn should_reject_request_bad_item_id() {
                let item_id = "some_item";
                let table_id = 1;

                let mut service = MockItemService::new();
                service.expect_get_item()
                    .never();

                let context = ItemRouterTestContext::new(service).await;

                let url = format!("http://{}/tables/{}/items/{}", context.address, table_id, item_id);
                let response = context.client.get(url)
                    .send()
                    .await
                    .expect("Failed to get response");

                let status = response.status();

                let body = response
                    .text()
                    .await
                    .expect("Failed to get body");

                assert_eq!(status, 422);
                assert_eq!(body, RAW_PATH_PARSE_ERROR);
            }
        }
        
        mod get_items_endpoint {
            async fn should_get_items() {
                
            }
            
            async fn should_get_empty_list() {
                
            }
            
            async fn should_reject_request_bad_table_id() {
                
            }
        }
        
        mod create_items_endpoint {
            async fn should_create_items() {
                
            }
            
            
            async fn should_reject_request_bad_table_id() {
                
            }
            
            async fn should_reject_request_no_body() {
                
            }
            
            async fn should_reject_request_bad_body() {
                
            }
        }
        
        mod delete_item_endpoint {
            async fn should_delete_item() {
                
            }
            
            async fn should_reject_request_bad_table_id() {
                
            }
            
            async fn should_reject_request_bad_item_id() {
                
            }
        }
    }
}