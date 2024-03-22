# naive-admin-rust


### 本项目是基于 [vue-naive-admin 2.0版](https://github.com/zclzone/vue-naive-admin)前端框架的 rust 版本做服务端

使用 axum session sqlx mysql 等技术栈

每天更新一点点，其中肯定有需要优化的地方，请指正

[api documnet](https://apifox.com/apidoc/shared-ff4a4d32-c0d1-4caf-b0ee-6abc130f734a/api-134496720)

[api demo code](https://gitee.com/-/ide/project/isme-admin/isme-nest-serve/edit/main/-/src/modules/role/dto.ts)

[web demo](https://admin.isme.top/login?redirect=/)

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
其他
### 基于 [vue-naive-admin 2.0版](https://github.com/zclzone/vue-naive-admin) 前端框架的 [goland 版本](https://github.com/ituserxxx/naive-admin-go)做服务端

