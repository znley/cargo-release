use std::env::VarError;
use std::io::Error as IOError;
use std::str::Utf8Error;
use std::string::FromUtf8Error;

use cargo_metadata::Error as CargoMetaError;
use regex::Error as RegexError;
use semver::SemVerError;
use toml::de::Error as TomlError;
use toml_edit::TomlError as TomlEditError;

quick_error! {
    #[derive(Debug)]
    pub enum FatalError {
        IOError(err: IOError) {
            from()
            cause(err)
            display("IO Error: {}", err)
        }
        InvalidCargoFileFormat(err: TomlError) {
            display("Invalid TOML file format: {}", err)
            from()
            cause(err)
        }
        InvalidCargoFileFormat2(err: TomlEditError) {
            display("Invalid TOML file format: {}", err)
            from()
            cause(err)
        }
        InvalidCargoFileFormat3(err: CargoMetaError) {
            display("Invalid TOML file format: {}", err)
            from()
            cause(err)
        }
        InvalidCargoConfigKeys {
            display("Invalid cargo-release config item found")
        }
        SemVerError(err: SemVerError) {
            from()
            cause(err)
            display("SemVerError {}", err)
        }
        IgnoreError(err: ignore::Error) {
            from()
            cause(err)
            display("ignore-pattern {}", err)
        }
        Utf8Error(err: Utf8Error) {
            from()
            cause(err)
            display("Utf8Error {}", err)
        }
        FromUtf8Error(err: FromUtf8Error) {
            from()
            cause(err)
            display("FromUtf8Error {}", err)
        }
        NoPackage {
            display("No package in manifest file")

        }
        InvalidReleaseLevel(level: String) {
            display("Unsupported release level {}, only major, minor and patch are supported", level)
        }
        UnsupportedPrereleaseVersionScheme {
            display("This version scheme is not supported by cargo-release. Use format like `pre`, `dev` or `alpha.1` for prerelease symbol")
        }
        UnsupportedVersionReq(req: String) {
            display("Support for modifying {} is currently unsupported", req)
        }
        ReplacerConfigError {
            display("Insuffient replacer config: file, search and replace are required.")
        }
        ReplacerRegexError(err: RegexError) {
            from()
            cause(err)
            display("RegexError {}", err)
        }
        ReplacerMinError(pattern: String, req: usize, actual: usize) {
            display("For `{}`, at least {} replacements expected, found {}", pattern, req, actual)
        }
        ReplacerMaxError(pattern: String, req: usize, actual: usize) {
            display("For `{}`, at most {} replacements expected, found {}", pattern, req, actual)
        }
        VarError(err: VarError) {
            from()
            cause(err)
            display("Environment Variable Error: {}", err)
        }
        GitError {
            display("git is not found. git is required for cargo-release workflow.")
        }
        PublishTimeoutError {
            display("Timeout waiting for crate to be published.")
        }
        DependencyVersionConflict {
            display("Dependency is configured to conflict with new version")
        }
    }
}
