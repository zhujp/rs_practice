1. 安装
cargo install diesel_cli --no-default-features --features mysql
diesel setup
diesel migration generate create_posts
2. 编辑迁移文件 `migmigrations`
3. 运行迁移
diesel migration run