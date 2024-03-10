# ipam-rust
一个iP地址管理平台
使用rust语言，
前端使用Bootstrap、Datatable、sweetalert，
后端使用Flask、APScheduler 框架，
数据库使用SQLite，
内置功能
	首页：没有首页的首页
	IP地址表：显示IP地址使用情况，可在页面进行增、删、改、导出到Excel表格
	分组：按分组 + 网段显示目录树，右侧显示所选择查看网段的饼图，方块表和网段IP地址表
	设置：查看定时任务执行情况和轮询参数
 -------
 项目结构
 ```
├── Cargo.toml
├── README.md
├── src
│   ├── main.rs
│   ├── models.rs
│   ├── routes.rs
│   └── utils.rs
│   ├── templates
│   │   ├── base.html
│   │   ├── index.html
│   │   ├── ip_addresses.html
│   │   ├── groups.html
│   │   └── settings.html
│   └── static
│       ├── css
│       │   └── bootstrap.min.css
│       │   └── datatables.min.css
│       ├── js
│       │   └── datatables.min.js
│       └── sweetalert2.min.js
```
---
主要文件
Cargo.toml: 项目配置文件，定义项目依赖的库和版本
main.rs: 程序入口文件，负责初始化应用程序并启动服务
models.rs: 定义数据库模型
routes.rs: 定义路由规则
utils.rs: 定义通用函数
templates: 存放前端页面模板
