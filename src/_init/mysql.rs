use crate::{
    config::{self, Config},
    constants::SQL_FILE_DIR,
    store::mysql,
};
use sqlx::{Error, MySqlPool};
use std::{fs, iter::Peekable, path::PathBuf, str::Chars};

pub async fn init(db_conn_url: &str) -> Result<(), Error> {
    // 获取配置
    let conf = config::get_config().await;

    // 重置库和用户
    drop_and_create_new_db(conf, db_conn_url).await?;

    // 执行 SQL 文件
    read_and_exec_sql_file(conf).await?;

    println!("初始化数据库 OK");

    Ok(())
}

// 重置库和用户
async fn drop_and_create_new_db(config: &Config, conn_url: &str) -> Result<(), Error> {
    // 创建连接池
    let root_db = mysql::new_mysql_pool(conn_url).await?;

    // 生成 SQL
    let sql_content = generate_drop_sql(config);

    // 执行 SQL 文件
    println!("初始化数据库 ...");
    exec_sql(&root_db, &sql_content).await?;

    Ok(())
}

// 读取并执行目录下的所有 SQL 文件
async fn read_and_exec_sql_file(config: &Config) -> Result<(), Error> {
    // 读取目录下的所有 SQL 文件
    let mut paths: Vec<PathBuf> = fs::read_dir(SQL_FILE_DIR)?
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .collect();

    // 按文件名称排序
    paths.sort();

    // 创建数据库连接池
    let db_pool = mysql::new_mysql_pool(&config.mysql.get_conn_url()).await?;

    // 执行所有 SQL 文件
    for file_path in paths
        .iter()
        .filter_map(|p| p.to_str())
        .filter(|p| p.ends_with(".sql"))
    {
        // Windows 路径分隔符替换
        let file_path = file_path.replace('\\', "/");

        // 读取并执行 SQL 文件
        println!("执行SQL文件: {file_path}");
        if let Err(err) = execute_sql_file(&db_pool, &file_path).await {
            println!("执行SQL文件 {file_path} 时出错: {err}");
            panic!("初始化数据库失败");
        }
    }

    Ok(())
}

// 执行 SQL 文件
async fn execute_sql_file(db: &MySqlPool, file_path: &str) -> Result<(), Error> {
    let content = fs::read_to_string(file_path)
        .map_err(|e| {
            println!("读取SQL文件 {file_path} 时出错: {e}");
        })
        .unwrap();

    exec_sql(db, &content).await
}

// 执行 SQL
async fn exec_sql(db: &MySqlPool, sql_content: &str) -> Result<(), Error> {
    // 去除注释
    let sql_content = remove_sql_comments(sql_content);

    // 分割 SQL 语句并逐条执行
    for sql in sql_content.split(';').filter_map(|s| {
        let trimmed = s.trim();
        if !trimmed.is_empty() {
            Some(trimmed)
        } else {
            None
        }
    }) {
        sqlx::query(sql).execute(db).await?;
    }

    Ok(())
}

// 根据配置文件生成 SQL
fn generate_drop_sql(config: &Config) -> String {
    // 缓存多次使用的配置字段
    let db_name = config.mysql.get_database();
    let user = config.mysql.get_username();
    let password = config.mysql.get_password();
    let charset = config.mysql.get_charset();
    let collate = config.mysql.get_collate();

    // 根据配置文件生成 SQL 内容
    format!(
        r#"
        DROP DATABASE IF EXISTS {db_name}; 
        DROP USER IF EXISTS '{user}'@'%'; 
        
        CREATE USER '{user}'@'%' IDENTIFIED BY '{password}'; 
        CREATE DATABASE {db_name} CHARACTER SET {charset} COLLATE {collate}; 
        
        GRANT ALL PRIVILEGES ON {db_name}.* TO '{user}'@'%'; 
        FLUSH PRIVILEGES;
        "#
    )
}

// 去除 SQL 注释的函数
fn remove_sql_comments(sql_content: &str) -> String {
    let mut result = String::with_capacity(sql_content.len());
    let mut chars = sql_content.chars().peekable();

    // 检查并跳过单行注释直到行尾
    fn skip_single_line_comment(chars: &mut Peekable<Chars>) {
        while let Some(&c) = chars.peek() {
            chars.next();
            if c == '\n' {
                break;
            }
        }
    }

    // 检查并跳过多行注释直到结束符 */
    fn skip_multi_line_comment(chars: &mut Peekable<Chars>) {
        while let Some(c) = chars.next() {
            if c == '*' {
                if let Some('/') = chars.peek() {
                    chars.next(); // 消费 '/'
                    break;
                }
            }
        }
    }

    while let Some(c) = chars.next() {
        match c {
            '-' if chars.peek() == Some(&'-') => {
                chars.next(); // 跳过第二个 '-'
                skip_single_line_comment(&mut chars);
            }
            '/' if chars.peek() == Some(&'*') => {
                chars.next(); // 跳过 '*'
                skip_multi_line_comment(&mut chars);
            }
            _ => result.push(c),
        }
    }

    result
}
