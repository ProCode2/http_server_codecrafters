[![progress-banner](https://backend.codecrafters.io/progress/http-server/b3e44ed1-95f6-4b1f-a4ad-a407bea47e11)](https://app.codecrafters.io/users/codecrafters-bot?r=2qF)

This is a starting point for Rust solutions to the
["Build Your Own HTTP server" Challenge](https://app.codecrafters.io/courses/http-server/overview).

[HTTP](https://en.wikipedia.org/wiki/Hypertext_Transfer_Protocol) is the
protocol that powers the web. In this challenge, you'll build a HTTP/1.1 server
that is capable of serving multiple clients.

Along the way you'll learn about TCP servers,
[HTTP request syntax](https://www.w3.org/Protocols/rfc2616/rfc2616-sec5.html),
and more.

## TODO
- [ ] Build a prefix tree for mapping urls to function
- [ ] Urls should be dynamic, meaning we should be able to pass `/file/{file_name}` and access `file_name` in the request object
