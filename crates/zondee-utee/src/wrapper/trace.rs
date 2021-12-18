//! Implementation of `log::Log`

use crate::wrapper::raw::{
    _utee_log as utee_log, trace_get_level, trace_set_level, TRACE_DEBUG, TRACE_ERROR, TRACE_FLOW,
    TRACE_INFO, TRACE_MIN,
};
use core::fmt;
use log::{Level, Metadata, Record};

#[derive(Default)]
pub struct TEELogger;

impl log::Log for TEELogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() >= self.level()
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let mut s = alloc::string::String::with_capacity(256);
            fmt::write(&mut s, *record.args()).expect("Bad formatting");
            s.push('\n');

            unsafe { utee_log(s.as_str().as_ptr() as *const _, s.as_str().len() as _) }
        }
    }

    fn flush(&self) {}
}

impl TEELogger {
    pub fn level(&self) -> Level {
        //probably not very adherent to the comment in trace.h
        match unsafe { trace_get_level() } as u32 {
            TRACE_MIN => Level::Error,
            TRACE_ERROR => Level::Warn,
            TRACE_INFO => Level::Info,
            TRACE_DEBUG => Level::Debug,
            TRACE_FLOW => Level::Trace,
            _ => Level::Error,
        }
    }

    pub fn set_level(&self, level: Level) {
        let level = match level {
            Level::Error => TRACE_ERROR,
            Level::Warn => TRACE_ERROR,
            Level::Info => TRACE_INFO,
            Level::Debug => TRACE_DEBUG,
            Level::Trace => TRACE_FLOW,
        };

        unsafe {
            trace_set_level(level as i32);
        }
    }

    ///Install this logger as the global logger
    pub fn install() -> Result<(), log::SetLoggerError> {
        unsafe { log::set_logger_racy(&Self {}) }?;
        log::set_max_level(Level::Trace.to_level_filter());
        Ok(())
    }
}
