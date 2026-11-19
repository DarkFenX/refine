pub(crate) enum ValResultFitFast {
    Passed,
    Failed,
    /// Failed potential drone bandwidth.
    ///
    /// Only validators which should check non-reduced drone bandwidth failed (using possibly
    /// reduced bandwidth). Sources of bandwidth reduction should be disabled, and validations
    /// retried.
    FailedPdbw,
}
