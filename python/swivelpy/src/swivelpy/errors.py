class SwivelError(RuntimeError):
    """Base error raised by swivelpy."""


class SwivelCommandError(SwivelError):
    """Raised when the swivel CLI exits with a non-zero status."""


class SwivelJsonError(SwivelError):
    """Raised when swivel returns invalid JSON."""


class SwivelConfigError(SwivelError):
    """Raised when required configuration is missing."""
