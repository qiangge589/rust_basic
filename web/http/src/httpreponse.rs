use std::collections::HashMap;
use std::io::{Result, Write};

// 定义一个表示 HTTP 响应的结构体
#[derive(Debug, PartialEq, Clone)]
pub struct HttpResponse<'a> {
    version: &'a str,                           // HTTP 版本号
    status_code: &'a str,                       // 状态码
    status_text: &'a str,                       // 状态文本
    headers: Option<HashMap<&'a str, &'a str>>, // 可选的头部信息
    body: Option<String>,                       // 可选的响应体
}

// HttpResponse 结构体的默认实现
impl<'a> Default for HttpResponse<'a> {
    fn default() -> Self {
        Self {
            version: "HTTP/1.1".into(),
            status_code: "200".into(),
            status_text: "OK".into(),
            headers: None,
            body: None,
        }
    }
}

// 将 HttpResponse 转换为 String 的实现
impl<'a> From<HttpResponse<'a>> for String {
    fn from(res: HttpResponse<'a>) -> String {
        let res1 = res.clone();
        format!(
            "{} {} {}\r\n{}Content-length: {}\r\n\r\n{}",
            &res1.version(),
            &res1.status_code(),
            &res1.status_text(),
            &res1.headers(),
            &res.body.unwrap().len(),
            &res1.body()
        )
    }
}

// HttpResponse 结构体的实现
impl<'a> HttpResponse<'a> {
    // 创建一个新的 HttpResponse
    pub fn new(
        status_code: &'a str,
        headers: Option<HashMap<&'a str, &'a str>>,
        body: Option<String>,
    ) -> HttpResponse<'a> {
        let mut response = HttpResponse::default();
        if status_code != "200" {
            response.status_code = status_code.into();
        };
        response.headers = match &headers {
            Some(_h) => headers,
            None => {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            }
        };
        response.status_text = match response.status_code {
            "200" => "OK".into(),
            "400" => "Bad Request".into(),
            "404" => "Not Found".into(),
            "500" => "Internal Server Error".into(),
            _ => "Not Found".into(),
        };
        response.body = body;
        response
    }

    // 将 HttpResponse 发送到指定的写入流
    pub fn send_response(&self, write_stream: &mut impl Write) -> Result<()> {
        let res = self.clone();
        let response_string: String = String::from(res);
        let _ = write!(write_stream, "{}", response_string);

        Ok(())
    }

    // 获取 HTTP 版本号
    fn version(&self) -> &str {
        self.version
    }

    // 获取状态码
    fn status_code(&self) -> &str {
        self.status_code
    }

    // 获取状态文本
    fn status_text(&self) -> &str {
        self.status_text
    }

    // 获取头部信息的字符串表示
    fn headers(&self) -> String {
        let map: HashMap<&str, &str> = self.headers.clone().unwrap();
        let mut header_string: String = "".into();
        for (k, v) in map.iter() {
            header_string = format!("{}{}: {}\r\n", header_string, k, v);
        }
        header_string
    }

    // 获取响应体
    pub fn body(&self) -> &str {
        match &self.body {
            Some(b) => b.as_str(),
            None => "",
        }
    }
}

// 测试代码
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response_struct_creation_200() {
        let response_actual = HttpResponse::new("200", None, Some("xxxx".into()));
        let response_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: "200",
            status_text: "OK",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("xxxx".into()),
        };

        assert_eq!(response_actual, response_expected);
    }

    #[test]
    fn test_response_struct_creation_404() {
        let response_actual = HttpResponse::new("404", None, Some("xxxx".into()));
        let response_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: "404",
            status_text: "Not Found",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("xxxx".into()),
        };

        assert_eq!(response_actual, response_expected);
    }

    #[test]
    fn test_http_response_creation() {
        let response_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: "404",
            status_text: "Not Found",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("xxxx".into()),
        };
        let http_string: String = response_expected.into();
        let actual_string =
            "HTTP/1.1 404 Not Found\r\nContent-Type: text/html\r\nContent-length: 4\r\n\r\nxxxx";
        assert_eq!(http_string, actual_string);
    }
}