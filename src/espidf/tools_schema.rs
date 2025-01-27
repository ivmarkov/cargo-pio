//!  This is an autogenerated file and includes small manual fixes
//!
//!  generated with [cargo-typify](https://github.com/oxidecomputer/typify) v0.1
//!  using command "cargo typify -B tools_schema.json"
//!  and the [tools_schema.json]( https://github.com/espressif/esp-idf/blob/master/tools/tools_schema.json ) file  
//!
//! manual adjustments:
//! - made everything pub into pub (crate)
//! - manual adding missing macos-arm64 prob to VersionInfo not included in schema -> https://github.com/espressif/esp-idf/issues/13853
//! - Switched out EnvVars with serde_json Value, cause the json schema is using a patternPropertie to "specify" the field key **sick**

#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::to_string_trait_impl)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[doc = r" Error types."]
pub(crate) mod error {
    #[doc = r" Error from a TryFrom or FromStr implementation."]
    pub(crate) struct ConversionError(std::borrow::Cow<'static, str>);
    impl std::error::Error for ConversionError {}
    impl std::fmt::Display for ConversionError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl std::fmt::Debug for ConversionError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
#[doc = "Array of strings. Used to represent paths (split into components) and command lines (split into arguments)"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Array of strings. Used to represent paths (split into components) and command lines (split into arguments)\","]
#[doc = "  \"type\": \"array\","]
#[doc = "  \"items\": {"]
#[doc = "    \"type\": \"string\""]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct ArrayOfStrings(pub(crate) Vec<String>);
impl std::ops::Deref for ArrayOfStrings {
    type Target = Vec<String>;
    fn deref(&self) -> &Vec<String> {
        &self.0
    }
}
impl From<ArrayOfStrings> for Vec<String> {
    fn from(value: ArrayOfStrings) -> Self {
        value.0
    }
}
impl From<&ArrayOfStrings> for ArrayOfStrings {
    fn from(value: &ArrayOfStrings) -> Self {
        value.clone()
    }
}
impl From<Vec<String>> for ArrayOfStrings {
    fn from(value: Vec<String>) -> Self {
        Self(value)
    }
}
#[doc = "Collection of environment variables. Keys and values are the environment variable names and values, respectively."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Collection of environment variables. Keys and values are the environment variable names and values, respectively.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"patternProperties\": {"]
#[doc = "    \"^([A-Z_0-9]+)+$\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
//#[serde(deny_unknown_fields)]
pub(crate) struct EnvVars(pub(crate) HashMap<String, String>);
impl From<EnvVars> for HashMap<String, String> {
    fn from(value: EnvVars) -> Self {
        value.0
    }
}
impl From<&EnvVars> for EnvVars {
    fn from(value: &EnvVars) -> Self {
        value.clone()
    }
}
impl From<HashMap<String, String>> for EnvVars {
    fn from(value: HashMap<String, String>) -> Self {
        Self(value)
    }
}
#[doc = "Array of paths to be exported (added to PATH). Each item in the array is relative to the directory where the tool will be installed."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Array of paths to be exported (added to PATH). Each item in the array is relative to the directory where the tool will be installed.\","]
#[doc = "  \"type\": \"array\","]
#[doc = "  \"items\": {"]
#[doc = "    \"$ref\": \"#/definitions/arrayOfStrings\""]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct ExportPaths(pub(crate) Vec<ArrayOfStrings>);
impl std::ops::Deref for ExportPaths {
    type Target = Vec<ArrayOfStrings>;
    fn deref(&self) -> &Vec<ArrayOfStrings> {
        &self.0
    }
}
impl From<ExportPaths> for Vec<ArrayOfStrings> {
    fn from(value: ExportPaths) -> Self {
        value.0
    }
}
impl From<&ExportPaths> for ExportPaths {
    fn from(value: &ExportPaths) -> Self {
        value.clone()
    }
}
impl From<Vec<ArrayOfStrings>> for ExportPaths {
    fn from(value: Vec<ArrayOfStrings>) -> Self {
        Self(value)
    }
}
#[doc = "If 'always', the tool will be installed by default. If 'on_request', tool will be installed when specifically requested. If 'never', tool will not be considered for installation."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"If 'always', the tool will be installed by default. If 'on_request', tool will be installed when specifically requested. If 'never', tool will not be considered for installation.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"always\","]
#[doc = "    \"on_request\","]
#[doc = "    \"never\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub(crate) enum InstallRequirementInfo {
    #[serde(rename = "always")]
    Always,
    #[serde(rename = "on_request")]
    OnRequest,
    #[serde(rename = "never")]
    Never,
}
impl From<&InstallRequirementInfo> for InstallRequirementInfo {
    fn from(value: &InstallRequirementInfo) -> Self {
        value.clone()
    }
}
impl ToString for InstallRequirementInfo {
    fn to_string(&self) -> String {
        match *self {
            Self::Always => "always".to_string(),
            Self::OnRequest => "on_request".to_string(),
            Self::Never => "never".to_string(),
        }
    }
}
impl std::str::FromStr for InstallRequirementInfo {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "always" => Ok(Self::Always),
            "on_request" => Ok(Self::OnRequest),
            "never" => Ok(Self::Never),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for InstallRequirementInfo {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for InstallRequirementInfo {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for InstallRequirementInfo {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Information about download artifact for one platform"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Information about download artifact for one platform\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"sha256\","]
#[doc = "    \"size\","]
#[doc = "    \"url\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"rename_dist\": {"]
#[doc = "      \"description\": \"Filename under which the archive should be downloaded\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"sha256\": {"]
#[doc = "      \"description\": \"SHA256 sum of the file\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"size\": {"]
#[doc = "      \"description\": \"Size of the file, in bytes\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"url\": {"]
#[doc = "      \"description\": \"Download URL\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct PlatformDownloadInfo {
    #[doc = "Filename under which the archive should be downloaded"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) rename_dist: Option<String>,
    #[doc = "SHA256 sum of the file"]
    pub(crate) sha256: String,
    #[doc = "Size of the file, in bytes"]
    pub(crate) size: i64,
    #[doc = "Download URL"]
    pub(crate) url: String,
}
impl From<&PlatformDownloadInfo> for PlatformDownloadInfo {
    fn from(value: &PlatformDownloadInfo) -> Self {
        value.clone()
    }
}
#[doc = "Platform-specific values which override the defaults"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Platform-specific values which override the defaults\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"platforms\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"export_paths\": {"]
#[doc = "      \"description\": \"Platform-specific replacement for toolInfo/export_paths\","]
#[doc = "      \"$ref\": \"#/definitions/exportPaths\""]
#[doc = "    },"]
#[doc = "    \"export_vars\": {"]
#[doc = "      \"description\": \"Platform-specific replacement for toolInfo/export_vars\","]
#[doc = "      \"$ref\": \"#/definitions/envVars\""]
#[doc = "    },"]
#[doc = "    \"install\": {"]
#[doc = "      \"description\": \"Platform-specific replacement for toolInfo/install\","]
#[doc = "      \"$ref\": \"#/definitions/installRequirementInfo\""]
#[doc = "    },"]
#[doc = "    \"platforms\": {"]
#[doc = "      \"description\": \"List of platforms to which this override applies\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\","]
#[doc = "        \"enum\": ["]
#[doc = "          \"linux-i686\","]
#[doc = "          \"linux-amd64\","]
#[doc = "          \"linux-armel\","]
#[doc = "          \"linux-arm64\","]
#[doc = "          \"macos\","]
#[doc = "          \"macos-arm64\","]
#[doc = "          \"win32\","]
#[doc = "          \"win64\""]
#[doc = "        ]"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"strip_container_dirs\": {"]
#[doc = "      \"description\": \"Platform-specific replacement for toolInfo/strip_container_dirs\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"supported_targets\": {"]
#[doc = "      \"description\": \"Platform-specific replacement for toolInfo/supported_targets\","]
#[doc = "      \"$ref\": \"#/definitions/arrayOfStrings\""]
#[doc = "    },"]
#[doc = "    \"version_cmd\": {"]
#[doc = "      \"description\": \"Platform-specific replacement for toolInfo/version_cmd\","]
#[doc = "      \"$ref\": \"#/definitions/arrayOfStrings\""]
#[doc = "    },"]
#[doc = "    \"version_regex\": {"]
#[doc = "      \"description\": \"Platform-specific replacement for toolInfo/version_regex\","]
#[doc = "      \"$ref\": \"#/definitions/regex\""]
#[doc = "    },"]
#[doc = "    \"version_regex_replace\": {"]
#[doc = "      \"description\": \"Platform-specific replacement for toolInfo/version_regex_replace\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct PlatformOverrideInfo {
    #[doc = "Platform-specific replacement for toolInfo/export_paths"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) export_paths: Option<ExportPaths>,
    #[doc = "Platform-specific replacement for toolInfo/export_vars"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) export_vars: Option<EnvVars>,
    #[doc = "Platform-specific replacement for toolInfo/install"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) install: Option<InstallRequirementInfo>,
    #[doc = "List of platforms to which this override applies"]
    pub(crate) platforms: Vec<PlatformOverrideInfoPlatformsItem>,
    #[doc = "Platform-specific replacement for toolInfo/strip_container_dirs"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) strip_container_dirs: Option<String>,
    #[doc = "Platform-specific replacement for toolInfo/supported_targets"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) supported_targets: Option<ArrayOfStrings>,
    #[doc = "Platform-specific replacement for toolInfo/version_cmd"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) version_cmd: Option<ArrayOfStrings>,
    #[doc = "Platform-specific replacement for toolInfo/version_regex"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) version_regex: Option<Regex>,
    #[doc = "Platform-specific replacement for toolInfo/version_regex_replace"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) version_regex_replace: Option<String>,
}
impl From<&PlatformOverrideInfo> for PlatformOverrideInfo {
    fn from(value: &PlatformOverrideInfo) -> Self {
        value.clone()
    }
}
#[doc = "PlatformOverrideInfoPlatformsItem"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"linux-i686\","]
#[doc = "    \"linux-amd64\","]
#[doc = "    \"linux-armel\","]
#[doc = "    \"linux-arm64\","]
#[doc = "    \"macos\","]
#[doc = "    \"macos-arm64\","]
#[doc = "    \"win32\","]
#[doc = "    \"win64\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub(crate) enum PlatformOverrideInfoPlatformsItem {
    #[serde(rename = "linux-i686")]
    LinuxI686,
    #[serde(rename = "linux-amd64")]
    LinuxAmd64,
    #[serde(rename = "linux-armel")]
    LinuxArmel,
    #[serde(rename = "linux-arm64")]
    LinuxArm64,
    #[serde(rename = "macos")]
    Macos,
    #[serde(rename = "macos-arm64")]
    MacosArm64,
    #[serde(rename = "win32")]
    Win32,
    #[serde(rename = "win64")]
    Win64,
}
impl From<&PlatformOverrideInfoPlatformsItem> for PlatformOverrideInfoPlatformsItem {
    fn from(value: &PlatformOverrideInfoPlatformsItem) -> Self {
        value.clone()
    }
}
impl ToString for PlatformOverrideInfoPlatformsItem {
    fn to_string(&self) -> String {
        match *self {
            Self::LinuxI686 => "linux-i686".to_string(),
            Self::LinuxAmd64 => "linux-amd64".to_string(),
            Self::LinuxArmel => "linux-armel".to_string(),
            Self::LinuxArm64 => "linux-arm64".to_string(),
            Self::Macos => "macos".to_string(),
            Self::MacosArm64 => "macos-arm64".to_string(),
            Self::Win32 => "win32".to_string(),
            Self::Win64 => "win64".to_string(),
        }
    }
}
impl std::str::FromStr for PlatformOverrideInfoPlatformsItem {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "linux-i686" => Ok(Self::LinuxI686),
            "linux-amd64" => Ok(Self::LinuxAmd64),
            "linux-armel" => Ok(Self::LinuxArmel),
            "linux-arm64" => Ok(Self::LinuxArm64),
            "macos" => Ok(Self::Macos),
            "macos-arm64" => Ok(Self::MacosArm64),
            "win32" => Ok(Self::Win32),
            "win64" => Ok(Self::Win64),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for PlatformOverrideInfoPlatformsItem {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PlatformOverrideInfoPlatformsItem {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PlatformOverrideInfoPlatformsItem {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A regular expression"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A regular expression\","]
#[doc = "  \"type\": \"string\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub(crate) struct Regex(pub(crate) String);
impl std::ops::Deref for Regex {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<Regex> for String {
    fn from(value: Regex) -> Self {
        value.0
    }
}
impl From<&Regex> for Regex {
    fn from(value: &Regex) -> Self {
        value.clone()
    }
}
impl From<String> for Regex {
    fn from(value: String) -> Self {
        Self(value)
    }
}
impl std::str::FromStr for Regex {
    type Err = std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ToString for Regex {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
#[doc = "Information about one tool"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Information about one tool\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"description\","]
#[doc = "    \"export_paths\","]
#[doc = "    \"info_url\","]
#[doc = "    \"install\","]
#[doc = "    \"license\","]
#[doc = "    \"version_cmd\","]
#[doc = "    \"version_regex\","]
#[doc = "    \"versions\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"A short (one sentence) description of the tool.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"export_paths\": {"]
#[doc = "      \"$ref\": \"#/definitions/exportPaths\""]
#[doc = "    },"]
#[doc = "    \"export_vars\": {"]
#[doc = "      \"description\": \"Some variable expansions are done on the values. 1) ${TOOL_PATH} is replaced with the directory where the tool is installed.\","]
#[doc = "      \"$ref\": \"#/definitions/envVars\""]
#[doc = "    },"]
#[doc = "    \"info_url\": {"]
#[doc = "      \"description\": \"URL of the page with information about the tool\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"install\": {"]
#[doc = "      \"description\": \"If 'always', the tool will be installed by default. If 'on_request', tool will be installed when specifically requested. If 'never', tool will not be considered for installation.\","]
#[doc = "      \"$ref\": \"#/definitions/installRequirementInfo\""]
#[doc = "    },"]
#[doc = "    \"is_executable\": {"]
#[doc = "      \"description\": \"If false - tool does not contain executables. The version will not be checked but export_vars applied.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"license\": {"]
#[doc = "      \"description\": \"License name. Use SPDX license identifier if it exists, short name of the license otherwise.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"Tool name (used as a directory name)\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"platform_overrides\": {"]
#[doc = "      \"description\": \"List of platform-specific overrides\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/platformOverrideInfo\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"strip_container_dirs\": {"]
#[doc = "      \"description\": \"If specified, this number of top directory levels will removed when extracting. E.g. if strip_container_dirs=2, archive path a/b/c/d.txt will be extracted as c/d.txt\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"supported_targets\": {"]
#[doc = "      \"description\": \"Array of esp_targets that this tool is needed for.\","]
#[doc = "      \"$ref\": \"#/definitions/arrayOfStrings\""]
#[doc = "    },"]
#[doc = "    \"version_cmd\": {"]
#[doc = "      \"description\": \"Command to be executed (along with any extra arguments). The executable be present in one of the export_paths.\","]
#[doc = "      \"$ref\": \"#/definitions/arrayOfStrings\""]
#[doc = "    },"]
#[doc = "    \"version_regex\": {"]
#[doc = "      \"description\": \"Regex which is to be applied to version_cmd output to extract the version. By default, the version will be the first capture group of the expression. If version_regex_replace is specified, version will be obtained by doing a substitution using version_regex_replace instead.\","]
#[doc = "      \"$ref\": \"#/definitions/regex\""]
#[doc = "    },"]
#[doc = "    \"version_regex_replace\": {"]
#[doc = "      \"description\": \"If given, this will be used as substitute expression for the regex defined in version_regex, to obtain the version string. Not specifying this is equivalent to setting it to '\\\\1' (i.e. return the first capture group).\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"versions\": {"]
#[doc = "      \"description\": \"List of versions\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/versionInfo\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct ToolInfo {
    #[doc = "A short (one sentence) description of the tool."]
    pub(crate) description: String,
    pub(crate) export_paths: ExportPaths,
    #[doc = "Some variable expansions are done on the values. 1) ${TOOL_PATH} is replaced with the directory where the tool is installed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) export_vars: Option<EnvVars>,
    #[doc = "URL of the page with information about the tool"]
    pub(crate) info_url: String,
    #[doc = "If 'always', the tool will be installed by default. If 'on_request', tool will be installed when specifically requested. If 'never', tool will not be considered for installation."]
    pub(crate) install: InstallRequirementInfo,
    #[doc = "If false - tool does not contain executables. The version will not be checked but export_vars applied."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) is_executable: Option<bool>,
    #[doc = "License name. Use SPDX license identifier if it exists, short name of the license otherwise."]
    pub(crate) license: String,
    #[doc = "Tool name (used as a directory name)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) name: Option<String>,
    #[doc = "List of platform-specific overrides"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(crate) platform_overrides: Vec<PlatformOverrideInfo>,
    #[doc = "If specified, this number of top directory levels will removed when extracting. E.g. if strip_container_dirs=2, archive path a/b/c/d.txt will be extracted as c/d.txt"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) strip_container_dirs: Option<i64>,
    #[doc = "Array of esp_targets that this tool is needed for."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) supported_targets: Option<ArrayOfStrings>,
    #[doc = "Command to be executed (along with any extra arguments). The executable be present in one of the export_paths."]
    pub(crate) version_cmd: ArrayOfStrings,
    #[doc = "Regex which is to be applied to version_cmd output to extract the version. By default, the version will be the first capture group of the expression. If version_regex_replace is specified, version will be obtained by doing a substitution using version_regex_replace instead."]
    pub(crate) version_regex: Regex,
    #[doc = "If given, this will be used as substitute expression for the regex defined in version_regex, to obtain the version string. Not specifying this is equivalent to setting it to '\\1' (i.e. return the first capture group)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) version_regex_replace: Option<String>,
    #[doc = "List of versions"]
    pub(crate) versions: Vec<VersionInfo>,
}
impl From<&ToolInfo> for ToolInfo {
    fn from(value: &ToolInfo) -> Self {
        value.clone()
    }
}
#[doc = "VersionInfo"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"any\": {"]
#[doc = "      \"$ref\": \"#/definitions/platformDownloadInfo\""]
#[doc = "    },"]
#[doc = "    \"linux-amd64\": {"]
#[doc = "      \"$ref\": \"#/definitions/platformDownloadInfo\""]
#[doc = "    },"]
#[doc = "    \"linux-arm64\": {"]
#[doc = "      \"$ref\": \"#/definitions/platformDownloadInfo\""]
#[doc = "    },"]
#[doc = "    \"linux-armel\": {"]
#[doc = "      \"$ref\": \"#/definitions/platformDownloadInfo\""]
#[doc = "    },"]
#[doc = "    \"linux-i686\": {"]
#[doc = "      \"$ref\": \"#/definitions/platformDownloadInfo\""]
#[doc = "    },"]
#[doc = "    \"macos\": {"]
#[doc = "      \"$ref\": \"#/definitions/platformDownloadInfo\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"Version name (used as a directory name)\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"status\": {"]
#[doc = "      \"description\": \"Determines whether the version is recommended/supported/deprecated\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"recommended\","]
#[doc = "        \"supported\","]
#[doc = "        \"deprecated\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"win32\": {"]
#[doc = "      \"$ref\": \"#/definitions/platformDownloadInfo\""]
#[doc = "    },"]
#[doc = "    \"win64\": {"]
#[doc = "      \"$ref\": \"#/definitions/platformDownloadInfo\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct VersionInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) any: Option<PlatformDownloadInfo>,
    #[serde(
        rename = "linux-amd64",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub(crate) linux_amd64: Option<PlatformDownloadInfo>,
    #[serde(
        rename = "linux-arm64",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub(crate) linux_arm64: Option<PlatformDownloadInfo>,
    #[serde(
        rename = "linux-armel",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub(crate) linux_armel: Option<PlatformDownloadInfo>,
    #[serde(
        rename = "linux-i686",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub(crate) linux_i686: Option<PlatformDownloadInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) macos: Option<PlatformDownloadInfo>,
    #[serde(
        rename = "macos-arm64",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub(crate) macos_arm64: Option<PlatformDownloadInfo>,
    #[doc = "Version name (used as a directory name)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) name: Option<String>,
    #[doc = "Determines whether the version is recommended/supported/deprecated"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) status: Option<VersionInfoStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) win32: Option<PlatformDownloadInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) win64: Option<PlatformDownloadInfo>,
}
impl From<&VersionInfo> for VersionInfo {
    fn from(value: &VersionInfo) -> Self {
        value.clone()
    }
}
#[doc = "Determines whether the version is recommended/supported/deprecated"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Determines whether the version is recommended/supported/deprecated\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"recommended\","]
#[doc = "    \"supported\","]
#[doc = "    \"deprecated\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub(crate) enum VersionInfoStatus {
    #[serde(rename = "recommended")]
    Recommended,
    #[serde(rename = "supported")]
    Supported,
    #[serde(rename = "deprecated")]
    Deprecated,
}
impl From<&VersionInfoStatus> for VersionInfoStatus {
    fn from(value: &VersionInfoStatus) -> Self {
        value.clone()
    }
}
impl ToString for VersionInfoStatus {
    fn to_string(&self) -> String {
        match *self {
            Self::Recommended => "recommended".to_string(),
            Self::Supported => "supported".to_string(),
            Self::Deprecated => "deprecated".to_string(),
        }
    }
}
impl std::str::FromStr for VersionInfoStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "recommended" => Ok(Self::Recommended),
            "supported" => Ok(Self::Supported),
            "deprecated" => Ok(Self::Deprecated),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for VersionInfoStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for VersionInfoStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for VersionInfoStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
