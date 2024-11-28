mod formatter;
mod init;

use crate::constants::REQUEST_ID_HEADER;
use axum::http::HeaderName;

pub use formatter::Formatter;
pub use init::init;

pub const X_REQUEST_ID: HeaderName = HeaderName::from_static(REQUEST_ID_HEADER);
