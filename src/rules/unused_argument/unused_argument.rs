
impl UnusedArgument {
    /// Create a new `UnusedArgument` lint.
    pub fn new() -> Self {
        Self {
            name: "unused_argument",
            default_level: Level::Warn,
            desc: "unused argument",
        }
    }
}