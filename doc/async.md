# Async support

The FUI library supports async code using `Tokio` runtime. To enable it, you must add `async` feature to the `fui_app` crate:

```cargo
fui_app = { version = "0.2", features = ["async"] }
```
