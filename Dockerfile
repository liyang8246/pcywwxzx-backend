# 使用rust:1-bullseye基础镜像
FROM rust:1-bullseye

# 设置工作目录
WORKDIR /usr/src/myapp

# 安装sqlite3和openssl
RUN apt-get update && apt-get install -y sqlite3 libsqlite3-dev openssl

# 复制项目源代码到工作目录
COPY . .

# 安装Python3（rust镜像可能不包含Python3）
RUN apt-get install -y python3

# 编译Rust项目
RUN SQLX_OFFLINE=true cargo build --release
RUN cp target/release/pcyw-salvo ./
RUN rm -rf target
# 运行python脚本并启动Rust应用程序
CMD ["sh", "-c", "./pcyw-salvo"]
