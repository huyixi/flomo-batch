# flomo-batch

📝 批量将 `.txt` 文件中的内容发送到 [Flomo](https://flomoapp.com)。
需要使用 flomo Api

---

## 用法

```bash
# 预览模式，只预览将要发送的内容
cargo run -- notes.txt -p

# 实际发送内容（需设置 FLOMO_WEBHOOK_URL 环境变量）
cargo run -- notes.txt
