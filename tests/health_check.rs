#[cfg(test)]
mod health_check_test {
    use actix_web::{test, web, App};

    #[actix_web::test]
    async fn test() {
        let app = test::init_service(
            App::new().route("/health_check", web::get().to(zero2prod::health_check)),
        )
        .await;
        let req = test::TestRequest::get().uri("/health_check").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}
