// Enum definition for user status
enum UserStatus {
    INACTIVE = 0,
    ACTIVE = 1,
    SUSPENDED = 2,
    BANNED = 3
}

// Enum definition for access levels
enum AccessLevel {
    GUEST_ACCESS = 1,
    READ_ONLY_ACCESS = 2,
    READ_WRITE_ACCESS = 3,
    ADMIN_ACCESS = 4
}

// Enum definition for log levels
enum LogLevel {
    DEBUG = 1
    INFO = 2
    WARN = 3
    ERROR = 4
    FATAL = 5
}