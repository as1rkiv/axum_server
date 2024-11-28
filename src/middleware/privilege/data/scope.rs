use serde::{de::Deserializer, ser::Serializer, Deserialize, Serialize};
use sqlx::{
    decode::Decode,
    error::BoxDynError,
    mysql::{MySql, MySqlTypeInfo},
    Database, Type,
};
use std::fmt;

/// 数据访问范围枚举
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum DataScope {
    /// 本人
    SelfScope = 0,
    /// 本部门
    DeptScope = 1,
    /// 下级部门
    HircScope = 2,
    /// 所有
    AllScope = 3,
}

// 默认
impl Default for DataScope {
    fn default() -> Self {
        Self::SelfScope
    }
}

// 显示
impl fmt::Display for DataScope {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", *self as u8)
    }
}

// 序列化
impl Serialize for DataScope {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u8(*self as u8)
    }
}

// 反序列化
impl<'de> Deserialize<'de> for DataScope {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match u8::deserialize(deserializer)? {
            1 => Ok(Self::DeptScope),
            2 => Ok(Self::HircScope),
            3 => Ok(Self::AllScope),
            _ => Ok(Self::default()),
        }
    }
}

// sqlx 识别 DataScope
impl Type<MySql> for DataScope {
    fn type_info() -> MySqlTypeInfo {
        <u8 as Type<MySql>>::type_info()
    }
}

// 数据库 TINYINT UNSIGNED 解码
impl<'r> Decode<'r, MySql> for DataScope {
    fn decode(value: <MySql as Database>::ValueRef<'r>) -> Result<Self, BoxDynError> {
        match <u8 as Decode<MySql>>::decode(value)? {
            1 => Ok(Self::DeptScope),
            2 => Ok(Self::HircScope),
            3 => Ok(Self::AllScope),
            _ => Ok(Self::default()),
        }
    }
}
