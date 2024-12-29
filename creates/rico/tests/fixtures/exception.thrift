// Exception for when a user is not found
exception UserNotFoundException {
    1: string message
}

// Exception for when there's an invalid input during user registration
exception InvalidUserRegistrationException {
    1: string errorMessage
    2: list<string> invalidFields
}

// Exception for when an order cannot be placed due to insufficient inventory
exception InsufficientInventoryException {
    1: string productName
    2: i32 requiredQuantity
    3: i32 availableQuantity
}

// Exception for when accessing a resource without proper authorization
exception UnauthorizedAccessException {
    1: string resourceName
    2: string userRole
}

// Exception for when a database operation fails
exception DatabaseOperationException {
    1: string operation
    2: string errorDetails
}

// Exception for when a network connection error occurs
exception NetworkConnectionException {
    1: string connectionType // e.g., "TCP", "HTTP"
    2: string errorMessage
}

// Exception for when an operation times out
exception OperationTimeoutException {
    1: string operationName
    2: i32 timeoutSeconds
}