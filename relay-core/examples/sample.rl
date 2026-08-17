[meta]
name = "httpbin get test"
type = "http"

[request]
method = "GET"
url = "https://httpbin.org/get?greeting={{greeting}}"

[request.headers]
"X-Relay-Test" = "true"
"Accept" = "application/json"
