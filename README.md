# AXUM 自用脚手架
```bash
# 单线程启动
`#[tokio::main(flavor = "current_thread")]`
# 多线程启动
`#[tokio::main(flavor = "multi_thread")]`
# 启动服务
cargo watch -q -c -w src/ -w config/ -w .cargo/ -x run
```
