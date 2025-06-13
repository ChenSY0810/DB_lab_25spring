# 如何使用

接着，你需要在根目录指定数据库，格式如下：
```bash
echo "DATABASE_URL=mysql://your_user:your_password@localhost:3306/DBLAB" > .env
```
接着运行`db.sql`里的所有语句，需要注意可能会删除SQL里的DBLAB数据库，最后在根目录下执行：
```bash
cargo run
```

最后，可以在127.0.0.1:3030访问网页。

*注*： 你需要保证服务端有权限调用typst才能使用导出pdf的功能。

# 使用框架

该作业使用了以下框架：
- Vue/Vite
- Rust: Warp

# 前端

- App.vue: 主程序
- router: 控制路由，路由守卫
- views: 不同窗口
- components: 组件

# 后端

- main.rs/ db.rs: 初始化前后端+数据库
- routes.rs: 处理路由并调用接口
- handlers.rs: 接口
- models.rs: 数据结构
- utils.rs: 一些简化代码的函数