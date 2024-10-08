## 广二师电脑义务维修中心后端系统

### 简介
本项目为广二师电脑义务维修中心的后端服务，采用 Rust 编程语言搭配 Salvo 框架和 SQLite 数据库构建，旨在为用户提供稳定、高效的服务支持

### 环境配置
在项目根目录下创建 .env 文件，并填入以下配置信息:

```
DATABASE_URL=sqlite://data.sqlite
MXNZP_APPID=mxnzp.com_AppID
MXNZP_SECRET=mxnzp.com_Secret
MANAGER_PASSWD=管理后台密码
PKCS12_PASSWD=xxx
```

DATABASE_URL:SQLite 数据库的连接字符串  
MXNZP_APPID 与 MXNZP_SECRET:API 服务所需的身份验证信息  
MANAGER_PASSWD:管理后台登录密码
PKCS12_PASSWD: 密钥文件密码

### 数据库结构
项目使用 SQLite 作为数据库系统，数据文件为 data.sqlite数据库中包含了一张名为 issue 的表，用于记录报修信息表结构如下:
```
CREATE TABLE IF NOT EXISTS issue (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    uid INTEGER NOT NULL,
    name TEXT NOT NULL,
    class TEXT NOT NULL,
    problem TEXT NOT NULL,
    reg_time TIMESTAMP NOT NULL,
    app_time TIMESTAMP NOT NULL,
    closed BOOLEAN NOT NULL,
    closed_time TIMESTAMP
);
```
id:报修单的唯一标识，自动生成  
uid:报修用户的唯一标识  
name:报修用户的姓名  
class:报修用户的班级  
problem:报修问题描述  
reg_time:报修单创建时间  
app_time:预约维修时间  
closed:报修单是否已关闭  
closed_time:报修单关闭时间  

### 运行与部署
安装 Rust 编程语言环境  
克隆本项目到本地  
在项目根目录下执行 cargo run --release 编译运行项目

### 许可证
本项目采用 [GPL-3.0](https://opensource.org/license/gpl-3-0) 许可证

---

感谢您使用广二师电脑义务维修中心后端系统，如有任何问题或建议，请随时联系我