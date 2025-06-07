# Detectrs
Check for new mysql version and send a mail if something changed.
## Version
started with Netlify, but no rust support
### aws with third party libs
- https://www.cargo-lambda.info/guide/getting-started.html
- https://github.com/awslabs/aws-lambda-rust-runtime
- use libsql (stack for debug must be extended) and turso.tech
- mailtrap.io used, smtp2go.com needs domain name
#### testing
```bash
cargo lambda watch
# other shell
cargo lambda invoke _ --data-ascii '{ "command": "hi" }'
```

## other
- https://github.com/dgtlmoon/changedetection.io
