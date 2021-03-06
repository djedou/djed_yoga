use internal;
use djed_self_tokenize_macro::SelfTokenize;
use djed_self_tokenize_trait::ToCustomTokens;
#[repr(u32)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, SelfTokenize)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub enum LogLevel {
	Error = 0,
	Warn = 1,
	Info = 2,
	Debug = 3,
	Verbose = 4,
	Fatal = 5,
}

impl From<LogLevel> for internal::YGLogLevel {
	fn from(l: LogLevel) -> internal::YGLogLevel {
		match l {
			LogLevel::Error => internal::YGLogLevel::YGLogLevelError,
			LogLevel::Warn => internal::YGLogLevel::YGLogLevelWarn,
			LogLevel::Info => internal::YGLogLevel::YGLogLevelInfo,
			LogLevel::Debug => internal::YGLogLevel::YGLogLevelDebug,
			LogLevel::Verbose => internal::YGLogLevel::YGLogLevelVerbose,
			LogLevel::Fatal => internal::YGLogLevel::YGLogLevelFatal,
		}
	}
}
