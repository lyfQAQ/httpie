# httpie
- 命令行解析
- 发送 http 请求
- 用户友好的方式输出

```
~/Desktop/work/github/httpie master !1 ❯ ./target/release/httpie post https://httpbin.org/post greeting=hola name=liuyifan
HTTP/1.1 200 OK

date: "Thu, 18 Apr 2024 03:20:16 GMT"
content-type: "application/json"
content-length: "512"
connection: "keep-alive"
server: "gunicorn/19.9.0"
access-control-allow-origin: "*"
access-control-allow-credentials: "true"


{
  "args": {},
  "data": "{\"name\":\"liuyifan\",\"greeting\":\"hola\"}",
  "files": {},
  "form": {},
  "headers": {
    "Accept": "*/*",
    "Content-Length": "37",
    "Content-Type": "application/json",
    "Host": "httpbin.org",
    "User-Agent": "Rust Httpie",
    "X-Amzn-Trace-Id": "Root=1-66209170-640b5558752fcb2376261a47",
    "X-Powerd-By": "Rust"
  },
  "json": {
    "greeting": "hola",
    "name": "liuyifan"
  },
  "origin": "103.182.96.100",
  "url": "https://httpbin.org/post"
}
```