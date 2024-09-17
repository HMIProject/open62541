impl super::MethodAttributes {
    #[must_use]
    pub const fn with_executable(mut self, executable: bool) -> Self {
        self.0.executable = executable;
        self
    }

    #[must_use]
    pub const fn with_user_executable(mut self, user_executable: bool) -> Self {
        self.0.userExecutable = user_executable;
        self
    }
}
