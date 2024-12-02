# 第一阶段：使用 rust:1-bookworm 作为编译环境
FROM rust:1-bookworm as builder

# 设置工作目录
WORKDIR /app

# 复制项目源代码到工作目录
COPY . .

# 设置环境变量
ENV SQLX_OFFLINE=true

# 编译Rust项目
RUN cargo build --release

# 第二阶段：使用 bookworm-slim 作为运行环境
FROM debian:bookworm-slim

# 设置工作目录
WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    libssl1.1 \
    && rm -rf /var/lib/apt/lists/*

# 从构建器阶段复制编译好的可执行文件
COPY --from=builder /app/target/release/pcyw-salvo ./

# 运行应用程序
CMD ["./pcyw-salvo"]
