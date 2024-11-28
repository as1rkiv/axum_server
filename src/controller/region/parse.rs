#![allow(dead_code)]

use crate::{common::error::Error, model::opt::region::TABLE_OPT_REGION, state::AppState};
use serde::{Deserialize, Serialize};
use tokio::{fs::File, io::AsyncReadExt};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonFile {
    label: String,
    value: String,
    children: Option<Vec<JsonFile>>,
}

pub async fn handle_file(app_state: &AppState) -> Result<(), Error> {
    let mut regions = load_file().await?;

    for item in regions.iter_mut() {
        change_code_to_len9(item).await?;
    }

    for item in regions {
        flatten_insert(app_state, None, item).await?;
    }

    Ok(())
}

async fn change_code_to_len9(root: &mut JsonFile) -> Result<(), Error> {
    let mut stack = vec![root];

    while let Some(node) = stack.pop() {
        // 插入当前节点
        if node.value.len() == 8 {
            // let s = node.value;
            node.value = format!("{}0{}", &node.value[..=5], &node.value[6..]);
        }

        // 将子节点压入栈
        if let Some(children) = node.children.as_mut() {
            for child in children.iter_mut() {
                stack.push(child);
            }
        }
    }

    Ok(())
}

async fn flatten_insert(
    app_state: &AppState,
    pid: Option<u64>,
    root: JsonFile,
) -> Result<(), Error> {
    let mut stack = vec![(pid, root)];

    while let Some((parent_id, node)) = stack.pop() {
        // 插入当前节点
        let res = sqlx::query(&format!(
            r#"INSERT INTO `{TABLE_OPT_REGION}` (`pid`, `name`, `code`) VALUES (?, ?, ?);"#
        ))
        .bind(parent_id)
        .bind(node.label)
        .bind(node.value)
        .execute(app_state.mysql())
        .await?;

        // 将子节点压入栈
        if let Some(children) = node.children {
            for child in children {
                stack.push((Some(res.last_insert_id()), child));
            }
        }
    }

    Ok(())
}

async fn load_file() -> Result<Vec<JsonFile>, Error> {
    let mut file = File::open("cities.json").await?;

    let mut content = String::new();

    file.read_to_string(&mut content).await?;

    Ok(serde_json::from_str(&content)?)
}
