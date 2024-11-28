mod conflict;
mod insert;

/// 递归
pub struct Recursion;

/// 递归插入通用接口
/// 表示一个可以构建树结构的节点
pub trait Recursive: Sized + Clone {
    /// 获取节点的唯一标识
    fn get_id(&self) -> i64;

    /// 获取节点的父节点标识，根节点返回 `None`
    fn get_pid(&self) -> Option<i64>;

    /// 获取子节点集合的可变引用
    fn get_children_mut(&mut self) -> &mut Option<Vec<Self>>;
}
