/// The format to use when exporting a `uv.lock` file.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
pub enum TreeFormat {
    /// Export in default tree format.
    #[default]
    #[serde(rename = "default", alias = "default")]
    #[cfg_attr(feature = "clap", clap(name = "default", alias = "default"))]
    Default,
    /// Export in json format.
    #[serde(rename = "json", alias = "json")]
    #[cfg_attr(feature = "clap", clap(name = "json", alias = "json"))]
    Json,
}
