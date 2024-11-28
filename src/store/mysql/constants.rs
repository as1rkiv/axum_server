#![allow(dead_code)]

/*
**  隔离级别
*/

/// 读未提交
/// 事务可以读取其他事务尚未提交的数据。
/// 这可能会导致脏读（dirty read）现象。
/// 即一个事务读取了另一个事务中尚未提交的数据，而该事务可能最终会回滚或修改数据，导致读取的数据不一致。
/// 适合对一致性要求极低的情况，几乎不常用，因为数据不安全。
pub const ISOLATION_READ_UNCOMMITTED: &str = r#"SET TRANSACTION ISOLATION LEVEL READ UNCOMMITTED"#;

/// 读已提交
/// 一个事务只能读取其他事务已经提交的数据，从而避免了脏读的问题。
/// 但是，不可重复读（non-repeatable read）问题仍然存在。
/// 即同一个事务中连续读取同一行数据时，如果其他事务进行了修改，读取的结果可能不一致。
/// 适用于大多数OLTP系统，既保持了数据一致性，又提升了并发性能。
pub const ISOLATION_READ_COMMITTED: &str = r#"SET TRANSACTION ISOLATION LEVEL READ COMMITTED"#;

/// 可重复读
/// 确保了事务内的多次读取结果一致，避免了脏读和不可重复读的问题。
/// 但在某些情况下，仍可能发生幻读（phantom read）。
/// 即在同一个事务中，当查询某个范围的数据时，如果另一个事务在该范围内插入了新数据，当前事务再次查询时会看到额外的数据。
/// 适合一致性要求较高但性能要求适中的应用。
pub const ISOLATION_REPEATABLE_READ: &str = r#"SET TRANSACTION ISOLATION LEVEL REPEATABLE READ"#;

/// 可串行化
/// 最高级别，通过强制事务按顺序执行来避免脏读、不可重复读和幻读等问题。
/// 通常实现方法是使用表级锁或行级锁，极大限制了并发性能。
/// 应用场景：数据一致性要求极高，但对并发性能要求较低的场景。
pub const ISOLATION_SERIALIZABLE: &str = r#"SET TRANSACTION ISOLATION LEVEL SERIALIZABLE"#;
