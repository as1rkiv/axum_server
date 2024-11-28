use super::{Recursion, Recursive};
use crate::common::error::Error;
use std::collections::{HashMap, HashSet, VecDeque};

impl Recursion {
    /// 将平铺的节点列表构造成嵌套的树状结构。
    /// 使用显式栈模拟递归构造，防止栈溢出
    ///
    /// # 参数
    /// - `list`: 包含所有节点的平铺列表，节点间的层级关系由 `id` 和 `pid` 表示。
    ///
    /// # 返回值
    /// 返回根节点的列表，其中每个根节点的 `children` 字段递归包含其子节点。
    ///
    /// # 实现细节
    /// - 使用 `HashMap` 将父节点与其子节点关联，提供快速查找子节点的能力。
    /// - 使用显式栈模拟递归，避免直接递归调用导致栈溢出问题。
    /// - 根节点的判断依据是其 `pid` 不属于 `list` 中的任何 `id`。
    pub fn insert_childrens<T: Recursive>(mut list: Vec<T>) -> Result<Vec<T>, Error> {
        // 空列表或无父子关系，直接返回
        if list.is_empty() || list.iter().all(|node| node.get_pid().is_none()) {
            return Ok(list);
        }

        // 检测循环依赖
        if let Err(cycle_nodes) = Self::detect_cycle(&list) {
            tracing::error!("Recursive 存在循环节点: {:?}", cycle_nodes);
            return Err(Error::Unavailable);
        }

        // 收集所有节点的 ID，作为候选的父节点集合。
        let parent_ids: HashSet<i64> = list.iter().map(T::get_id).collect();

        // 创建供插入子节点的 `HashMap` 和存储根节点的 `Vec`。
        let mut list_map: HashMap<i64, Vec<T>> = HashMap::with_capacity(list.len());
        let mut root_list = Vec::with_capacity(list.len().saturating_sub(parent_ids.len()));

        // 遍历传入的 `list`，根据 `pid` 判断节点是子节点还是根节点。
        for item in list.drain(..) {
            if let Some(pid) = item.get_pid() {
                // 如果 `pid` 在 `parent_ids` 中，说明是子节点，将其插入 `list_map`。
                if parent_ids.contains(&pid) {
                    list_map.entry(pid).or_default().push(item);
                    continue;
                }
            }

            // 否则，将其视为根节点，插入 `root_list`。
            root_list.push(item);
        }

        // 使用显式栈来模拟递归，处理节点的子节点插入。
        let mut stack: Vec<&mut T> = root_list.iter_mut().collect();
        while let Some(parent) = stack.pop() {
            // 根据 `parent` 的 `id` 查找其子节点。
            if let Some(children) = list_map.remove(&parent.get_id()) {
                // 将子节点插入 `parent` 的 `children` 字段。
                *parent.get_children_mut() = Some(children);

                // 将新插入的子节点继续压入栈，供后续处理，直到耗尽栈
                if let Some(parent_children) = parent.get_children_mut().as_mut() {
                    stack.extend(parent_children.iter_mut());
                }
            }
        }

        Ok(root_list)
    }

    /// 检查是否存在循环依赖
    /// 返回存在循环依赖的节点 ID
    /// 不假设所有父节点都存在
    fn detect_cycle<T: Recursive>(list: &[T]) -> Result<(), Vec<i64>> {
        let mut graph: HashMap<i64, Vec<i64>> = HashMap::with_capacity(list.len());
        let mut in_degree: HashMap<i64, usize> = HashMap::with_capacity(list.len());

        // 初始化图和入度表
        for node in list.iter() {
            let id = node.get_id();
            let pid = node.get_pid();

            // 确保节点在入度表中存在
            in_degree.entry(id).or_insert(0);

            // 如果有父节点，建立依赖关系
            if let Some(parent_id) = pid {
                // 如果父节点不在列表中，则跳过，但仍记录当前节点的入度
                if list.iter().any(|n| n.get_id() == parent_id) {
                    graph.entry(parent_id).or_default().push(id);
                    *in_degree.entry(id).or_insert(0) += 1;
                }
            }
        }

        // 收集所有入度为 0 的节点
        let mut queue: VecDeque<i64> = in_degree
            .iter()
            .filter_map(|(&id, &deg)| if deg == 0 { Some(id) } else { None })
            .collect();

        // 拓扑排序，记录处理过的节点
        let mut processed = HashSet::new();
        while let Some(node_id) = queue.pop_front() {
            processed.insert(node_id);

            // 遍历子节点，减少其入度
            if let Some(children) = graph.get(&node_id) {
                for &child_id in children {
                    if let Some(deg) = in_degree.get_mut(&child_id) {
                        *deg -= 1;
                        // 如果入度变为 0，则加入队列
                        if *deg == 0 {
                            queue.push_back(child_id);
                        }
                    }
                }
            }
        }

        // 如果处理过的节点数小于总节点数，说明有环
        if processed.len() < in_degree.len() {
            let cycle_nodes: Vec<i64> = in_degree
                .iter()
                .filter_map(|(&id, &deg)| if deg > 0 { Some(id) } else { None })
                .collect();
            Err(cycle_nodes)
        } else {
            Ok(())
        }
    }
}
