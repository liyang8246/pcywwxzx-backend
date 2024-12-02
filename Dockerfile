# 第一阶段：使用 rust:1-bullseye 作为编译环境
FROM rust:1-bullseye as builder

# 设置工作目录
WORKDIR /app

# 复制项目源代码到工作目录
COPY . .

# 安装交叉编译工具链
RUN rustup target add x86_64-unknown-linux-musl

# 编译Rust项目为静态链接的可执行文件
RUN SQLX_OFFLINE=true cargo build --release --target x86_64-unknown-linux-musl

# 第二阶段：使用 Alpine Linux 作为运行环境
FROM alpine:latest

# 设置工作目录
WORKDIR /app

# 从构建器阶段复制编译好的可执行文件
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/pcyw-salvo ./

# 运行应用程序
CMD ["./pcyw-salvo"]
