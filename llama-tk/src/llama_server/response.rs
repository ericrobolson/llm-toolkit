use std::{
    collections::HashMap,
    io::{Read, Write},
    net::TcpStream,
    process::Command,
    time::Duration,
};

const HTTP_REQUEST: &'static str = r#"POST /v1/chat/completions HTTP/1.1
Accept: */*
Accept-Encoding: gzip, deflate, br, zstd
Accept-Language: en-US,en;q=0.6
Cache-Control: no-cache
Connection: keep-alive
Content-Length: {BYTE_LENGTH}
Content-Type: application/json
Host: 127.0.0.1:8080
Origin: http://127.0.0.1:8080
Pragma: no-cache
Referer: http://127.0.0.1:8080/
Sec-Fetch-Dest: empty
Sec-Fetch-Mode: cors
Sec-Fetch-Site: same-origin
Sec-GPC: 1
User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/136.0.0.0 Safari/537.36
sec-ch-ua: "Chromium";v="136", "Brave";v="136", "Not.A/Brand";v="99"
sec-ch-ua-mobile: ?0
sec-ch-ua-platform: "macOS"

{BODY}
"#;

const HTTP_REQUEST_BODY: &'static str = r#"
{"messages":[{"role":"system","content":"You are a helpful assistant."},{"role":"user","content":"sah hi"}],"stream":true,"cache_prompt":true,"samplers":"edkypmxt","temperature":0.8,"dynatemp_range":0,"dynatemp_exponent":1,"top_k":40,"top_p":0.95,"min_p":0.05,"typical_p":1,"xtc_probability":0,"xtc_threshold":0.1,"repeat_last_n":64,"repeat_penalty":1,"presence_penalty":0,"frequency_penalty":0,"dry_multiplier":0,"dry_base":1.75,"dry_allowed_length":2,"dry_penalty_last_n":-1,"max_tokens":-1,"timings_per_token":false}
"#;

pub struct Response {
    response: String,
    completed: bool,
    stream: Option<TcpStream>,
}
impl Response {
    pub fn begin(address: &str, prompt: &str) -> Self {
        let mut stream = TcpStream::connect(address).unwrap();

        let body = HTTP_REQUEST_BODY;
        let byte_length = body.len();
        let request = HTTP_REQUEST.replace("{BYTE_LENGTH}", &byte_length.to_string());
        let request = request
            .replace("{BODY}", body)
            .replace("\r\n", "\n")
            .replace("\n", "\r\n");
        stream.write_all(request.as_bytes()).unwrap();

        stream.flush().unwrap();

        Self {
            response: String::new(),
            completed: false,
            stream: Some(stream),
        }
    }

    pub fn poll(&mut self) -> Status {
        if self.completed {
            return Status::Complete;
        }

        // This is all gross but we read from the TCP stream in chunks so we can output the response as it comes in.
        if let Some(stream) = &mut self.stream {
            let mut buf = [0; 1024];

            stream
                .set_read_timeout(Some(Duration::from_secs(1)))
                .unwrap();

            if let Ok(n) = stream.read(&mut buf) {
                if n > 0 {
                    let response = String::from_utf8_lossy(&buf[..n])
                        .to_string()
                        .replace("\r\n", "\n")
                        .to_string();

                    if response.contains("data:") {
                        let msg = response
                            .split("data:")
                            .nth(1)
                            .unwrap()
                            .split("\n")
                            .nth(0)
                            .unwrap()
                            .to_string()
                            .trim()
                            .to_string();

                        if msg == "[DONE]" {
                            self.completed = true;
                            return Status::Complete;
                        }

                        let json: HashMap<String, serde_json::Value> =
                            serde_json::from_str(&msg).unwrap();

                        let choices = json.get("choices").unwrap().as_array().unwrap();
                        for choice in choices {
                            // Check if it's done
                            if Some(serde_json::Value::String("stop".to_string()))
                                == choice.get("finish_reason").cloned()
                            {
                                self.completed = true;
                                return Status::Complete;
                            }

                            // Otherwise get the content
                            let delta = choice.get("delta").unwrap().as_object().unwrap();
                            let content = delta.get("content");
                            if let Some(content) = content {
                                if let Some(content) = content.as_str() {
                                    let content = content.to_string();
                                    self.response.push_str(&content);
                                    return Status::Poll { msg: Some(content) };
                                }
                            }
                        }
                    }

                    return Status::Poll { msg: None };
                }

                if n == 0 {
                    self.completed = true;
                }
            }
        }

        if self.completed {
            return Status::Complete;
        }

        Status::Poll { msg: None }
    }

    pub fn response(&self) -> Option<&str> {
        if self.completed {
            Some(&self.response)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    Poll { msg: Option<String> },
    Error { msg: String },
    Complete,
}
