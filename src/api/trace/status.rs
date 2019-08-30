#[derive(Debug, PartialEq, Eq, Clone)]
pub enum CanonicalCode {
    Ok = 0,
    Cancelled = 1,
    Unknown = 2,
    Argument = 3,
    DeadlineExceeded = 4,
    NotFound = 5,
    AlreadyExists = 6,
    PermissionDenied = 7,
    ResourceExhausted = 8,
    FailedPrecondition = 9,
    Aborted = 10,
    OutOfRange = 11,
    Unimplemented = 12,
    Internal = 13,
    Unavailable = 14,
    DataLoss = 15,
    Unauthenticated = 16,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Status {
    canonical_code: CanonicalCode,
    description: Option<String>,
}

impl Status {
    pub fn with_description(self, description: String) -> Self {
        Self {
            canonical_code: self.canonical_code,
            description: Some(description),
        }
    }

    pub fn ok() -> Self {
        Self {
            canonical_code: CanonicalCode::Ok,
            description: None,
        }
    }
    pub fn cancelled() -> Self {
        Self {
            canonical_code: CanonicalCode::Cancelled,
            description: None,
        }
    }
    pub fn unknown() -> Self {
        Self {
            canonical_code: CanonicalCode::Unknown,
            description: None,
        }
    }
    pub fn argument() -> Self {
        Self {
            canonical_code: CanonicalCode::Argument,
            description: None,
        }
    }
    pub fn deadline_exceeded() -> Self {
        Self {
            canonical_code: CanonicalCode::DeadlineExceeded,
            description: None,
        }
    }
    pub fn not_found() -> Self {
        Self {
            canonical_code: CanonicalCode::NotFound,
            description: None,
        }
    }
    pub fn already_exists() -> Self {
        Self {
            canonical_code: CanonicalCode::AlreadyExists,
            description: None,
        }
    }
    pub fn permission_denied() -> Self {
        Self {
            canonical_code: CanonicalCode::PermissionDenied,
            description: None,
        }
    }
    pub fn resource_exhausted() -> Self {
        Self {
            canonical_code: CanonicalCode::ResourceExhausted,
            description: None,
        }
    }
    pub fn failed_precondition() -> Self {
        Self {
            canonical_code: CanonicalCode::FailedPrecondition,
            description: None,
        }
    }
    pub fn aborted() -> Self {
        Self {
            canonical_code: CanonicalCode::Aborted,
            description: None,
        }
    }
    pub fn out_of_range() -> Self {
        Self {
            canonical_code: CanonicalCode::OutOfRange,
            description: None,
        }
    }
    pub fn unimplemented() -> Self {
        Self {
            canonical_code: CanonicalCode::Unimplemented,
            description: None,
        }
    }
    pub fn internal() -> Self {
        Self {
            canonical_code: CanonicalCode::Internal,
            description: None,
        }
    }
    pub fn unavailable() -> Self {
        Self {
            canonical_code: CanonicalCode::Unavailable,
            description: None,
        }
    }
    pub fn data_loss() -> Self {
        Self {
            canonical_code: CanonicalCode::DataLoss,
            description: None,
        }
    }
    pub fn unauthenticated() -> Self {
        Self {
            canonical_code: CanonicalCode::Unauthenticated,
            description: None,
        }
    }

    pub fn is_ok(&self) -> bool {
        self.canonical_code == CanonicalCode::Ok
    }
}
