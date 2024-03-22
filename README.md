# naive-admin-rust

基于 vue-naive-admin 2.0版 前端框架的 rust 版本做服务端

使用 axum session sqlx mysql 等技术栈

每天更新一点点，其中肯定有需要优化的地方，请指正

### 启动前端
配置前端接口地址

```shell
cd vue-naive-admin
```
修改文件 .env.development 里面的 VITE_PROXY_TARGET 为主机地址
#### 启动前端 web
```shell
npm i
npm run dev
```

### 启动后端服务

先配置 mysql 链接

修改项目根目录下 .env 文件里面的 DATABASE_URL 数据库地址

#### 启动服务
```shell
cargo run --bin main
```