use mysql::serde_json;
use reqwest::Client;
use reqwest::StatusCode;

// curl -X GET -H "Content-Type: application/json" http://127.0.0.1:8001/hello/jwt_en
// curl -X POST -H "Content-Type: application/json" -d '{"token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1aWQiOjIxLCJleHAiOjE3MDg5MzI5MDZ9.G3EPiI-hs8iRV8zMytR0Yk66nXuQ30KE280xtTMTUEg"}' "http://127.0.0.1:8001/hello/jwt_dn"

// curl -X GET -H "Content-Type: application/json" -H "Authorization: xxxxxxxxx" http://127.0.0.1:8061/h/a

const URL: &str = "http://127.0.0.1:8001/";

//详细 test 请求加参数 RUST_BACKTRACE=full cargo test --test http_test user_list
//详细 test 请求加参数 RUST_BACKTRACE=1 cargo test --test http_test user_list

// cargo test --test http_test root
#[tokio::test]
async fn root() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client.get(format!("{}", URL)).send().await?;
    assert_eq!(response.status(), StatusCode::OK);
    // 根据需要处理响应的内容
    let body = response.text().await?;
    println!("Response body: {}", body); // 在这里打印响应内容
    Ok(())
}

// curl -X POST -H "Content-Type: application/json" http://127.0.0.1:8001/user/list
// cargo test --test http_test user_list
#[tokio::test]
async fn user_list() {
    let client = Client::new();
    let body = ""; // POST 请求的 body 数据
    let response = client
        .post(format!("{}{}", URL, "user/list"))
        .body(body)
        .send()
        .await
        .expect("Failed to send POST request");
    let status_code = response.status();
    println!("Response status code: {}", status_code);

    let response_body = response.text().await.expect("Failed to read response body");
    println!("Response body: {}", response_body); // 打印响应内容

    assert!(
        status_code.is_success(),
        "Request failed with status code: {}",
        status_code
    );
}

// curl -X POST -H "Content-Type: application/json" -d '{"username": "xx", "password": "xx"}' http://127.0.0.1:8001/user/add
// cargo test --test http_test user_add
#[tokio::test]
async fn user_add() {
    let client = Client::new();
    let body = serde_json::json!({
        "password": "123456"
    });
    let response = client
        .post(format!("{}{}", URL, "user/add"))
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(body.to_string())
        .send()
        .await
        .expect("Failed to send POST request");
    let status_code = response.status();
    println!("Response status code: {}", status_code);

    let response_body = response.text().await.expect("Failed to read response body");
    println!("Response body: {}", response_body); // 打印响应内容

    assert!(
        status_code.is_success(),
        "Request failed with status code: {}",
        status_code
    );
}

// curl -X POST -H "Content-Type: application/json" -d '{"id": 10}' http://127.0.0.1:8001/user/info
// cargo test --test http_test user_info
#[tokio::test]
async fn user_info() {
    let client = Client::new();
    let body = serde_json::json!({
        "id": 1,
    });
    let response = client
        .post(format!("{}{}", URL, "user/info"))
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(body.to_string())
        .send()
        .await
        .expect("Failed to send POST request");
    let status_code = response.status();
    println!("Response status code: {}", status_code);

    let response_body = response.text().await.expect("Failed to read response body");
    println!("Response body: {}", response_body); // 打印响应内容

    assert!(
        status_code.is_success(),
        "Request failed with status code: {}",
        status_code
    );
}

// curl -X POST -H "Content-Type: application/json" -d '{"id": 11}' http://127.0.0.1:8001/user/del
// cargo test --test http_test user_del
#[tokio::test]
async fn user_del() {
    let client = Client::new();
    let body = serde_json::json!({
        "id": 1,
    });
    let response = client
        .post(format!("{}{}", URL, "user/del"))
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(body.to_string())
        .send()
        .await
        .expect("Failed to send POST request");
    let status_code = response.status();
    println!("Response status code: {}", status_code);

    let response_body = response.text().await.expect("Failed to read response body");
    println!("Response body: {}", response_body); // 打印响应内容

    assert!(
        status_code.is_success(),
        "Request failed with status code: {}",
        status_code
    );
}
