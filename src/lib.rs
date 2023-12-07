//Copyright (c) 2023 Volker Kleinfeld

use serde::{Deserialize, Serialize};
use std::fmt::Display;
use tracing::instrument;
use uuid::Uuid;


lazy_static::lazy_static! {
    static ref IDGEN: IdGen = IdGen::new();
}

#[derive(Debug)]
pub struct IdGen {}
impl IdGen {
    fn new() -> Self {
        Self {}
    }
    fn get(&self) -> String {
        Uuid::new_v4().to_string()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Location {
    pub file: String,
    pub line: u32,
    pub column: u32,
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "file: {}, line: {}, column: {}",
            self.file, self.line, self.column
        ))
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ErrorInfo {
    pub errorid: String,
    pub location: Location,
}

impl Display for ErrorInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "errorid: {}, location: ({})",
            self.errorid, self.location
        ))
    }
}

impl ErrorInfo {
    #[instrument(skip_all)]
    pub fn errorinfo(file: &str, line: u32, column: u32) -> ErrorInfo {
        let errorid = IDGEN.get();
        //tracing::error!(errorid, file, line, column, "collecting error information" );
        ErrorInfo {
            errorid,
            location: Location {
                file: file.to_string(),
                line,
                column,
            },
        }
    }
}

#[macro_export]
macro_rules! info {
    () => {
        $crate::ErrorInfo::errorinfo(file!(), line!(), column!())
    };
}

#[macro_export]
macro_rules! error {
    () => {
        $crate::ErrorInfo::errorinfo(file!(), line!(), column!())
    };
}

#[macro_export]
macro_rules! warn {
    () => {
        $crate::ErrorInfo::errorinfo(file!(), line!(), column!())
    };
}

#[macro_export]
macro_rules! debug {
    () => {
        $crate::ErrorInfo::errorinfo(file!(), line!(), column!())
    };
}

#[macro_export]
macro_rules! trace {
    () => {
        $crate::ErrorInfo::errorinfo(file!(), line!(), column!())
    };
}

#[cfg(feature = "axum")]
pub mod axum;
#[cfg(feature = "axum")]
pub use axum::err_resp;

#[cfg(test)]
mod tests {
    use crate::*;

    #[derive(thiserror::Error, Debug)]
    pub enum EnumError {
        #[error("this error happend here")]
        HappendHere(ErrorInfo),
        #[error("this error happend some where")]
        HappendSomeWhere(ErrorInfo, #[source] SomeError),
    }

    #[derive(thiserror::Error, Debug)]
    #[error("some base error")]
    pub struct SomeError;

    #[tracing::instrument(ret, err)]
    fn function1(var: u32) -> Result<u32, EnumError> {
        tracing::info!("now in function 1!");

        let var = function2(var)? - 1;
        if var == 0 {
            return Err(EnumError::HappendHere(info!()));
        }
        Ok(var)
    }

    #[tracing::instrument(ret, err)]
    fn function2(var: u32) -> Result<u32, EnumError> {
        tracing::info!("now in function 2!");

        let var = function3(var).map_err(|e| EnumError::HappendSomeWhere(info!(), e))? - 1;

        if var == 0 {
            return Err(EnumError::HappendHere(info!()));
        }
        Ok(var)
    }

    #[tracing::instrument(ret, err)]
    fn function3(var: u32) -> Result<u32, SomeError> {
        tracing::info!("now in function 3!");

        let var = var - 1;
        if var == 0 {
            return Err(SomeError);
        }
        Ok(var)
    }

    #[test]
    fn trace_errors() {
        //init_error_tracer(true);
        let traceid = IDGEN.get();
        let span = tracing::span!(tracing::Level::INFO, "span1", traceid = traceid);
        let _enter = span.enter();

        let r = function1(1);
        let e = if let Err(e) = r { e } else { panic!() };
        assert_eq!("this error happend some where", e.to_string());
        drop(_enter);
        let traceid = IDGEN.get();
        let span = tracing::span!(tracing::Level::INFO, "span2", traceid = traceid);
        let _enter = span.enter();
        let r = function1(2);
        let e = if let Err(e) = r { e } else { panic!() };
        assert_eq!("this error happend here", e.to_string());
        drop(_enter);
        let traceid = IDGEN.get();
        let span = tracing::span!(tracing::Level::INFO, "span3", traceid = traceid);
        let _enter = span.enter();
        let r = function1(3);
        let e = if let Err(e) = r { e } else { panic!() };
        assert_eq!("this error happend here", e.to_string());
    }
}
