use http::HeaderMap;
use reqwest::Client;
use url::Url;
use worker::*;

#[event(fetch)]
async fn fetch(mut req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    // init worker error panic
    console_error_panic_hook::set_once();

    // handle /favicon.ico
    if req.url()?.path() == "/favicon.ico" {
        let favicon = include_bytes!("../assets/favicon.ico");
        let mut response = Response::from_bytes(favicon.to_vec())?;
        response.headers_mut().set("Content-Type", "image/x-icon")?;
        return Ok(response);
    }

    // mapping method from `http` to `reqwest`
    let req_method = match req.method() {
        Method::Get => reqwest::Method::GET,
        Method::Post => reqwest::Method::POST,
        Method::Put => reqwest::Method::PUT,
        Method::Delete => reqwest::Method::DELETE,
        Method::Head => reqwest::Method::HEAD,
        Method::Options => reqwest::Method::OPTIONS,
        Method::Connect => reqwest::Method::CONNECT,
        Method::Patch => reqwest::Method::PATCH,
        Method::Trace => reqwest::Method::TRACE,
    };

    // get request body & url
    let req_url = match get_url_from_path(&req.url()?) {
        Ok(v) => v,
        Err(_) => return Response::error("Invalid URL", 500),
    };
    let req_header = HeaderMap::from(req.headers());
    let req_body = req.bytes().await.unwrap_or_default();

    // send request to the target server
    let res = Client::new()
        .request(req_method, req_url)
        .headers(req_header)
        .body(req_body)
        .send()
        .await
        .unwrap();

    // Save status and headers before consuming response
    let res_status = res.status().as_u16();
    let res_headers: Vec<(String, String)> = res
        .headers()
        .iter()
        .filter_map(|(name, value)| {
            // change `text/html` to `text/plain` for `content-type`
            // to avoid browser "Dangerous" web label
            if name.to_string().to_lowercase().eq("content-type")
                && value
                    .to_str()
                    .unwrap()
                    .to_lowercase()
                    .starts_with("text/html")
            {
                return value.to_str().ok().map(|v| {
                    (
                        name.to_string(),
                        "text/plain ; charset=\"utf-8\"".to_string(),
                    )
                });
            }
            value
                .to_str()
                .ok()
                .map(|v| (name.to_string(), v.to_string()))
        })
        .collect();

    // Now consume the response to get the body
    let res_body = res.bytes().await.unwrap();
    let mut response = Response::from_bytes(res_body.to_vec())?.with_status(res_status);

    // Set the saved headers
    for (name, value) in res_headers {
        response.headers_mut().set(&name, &value)?;
    }

    // Set CORS headers
    response
        .headers_mut()
        .set("access-control-allow-origin", "*")?;
    response
        .headers_mut()
        .set("access-control-allow-methods", "*")?;
    Ok(response)
}

fn get_url_from_path(url: &Url) -> Result<String> {
    let path = &url.path()[1..];
    if path.is_empty() {
        return Err("Invalid URL".into());
    }
    let query = match url.query() {
        Some(q) => format!("?{q}"),
        None => "".to_string(),
    };
    let full_url = format!("{path}{query}");
    if full_url.starts_with("http://") || full_url.starts_with("https://") {
        return Ok(full_url);
    }
    Ok(format!("https://{full_url}"))
}
