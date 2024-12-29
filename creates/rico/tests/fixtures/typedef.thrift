// Typedef for a user ID, which is an integer type
typedef i32 UserId
const UserId foo=12
const UserId bar=14

// Typedef for a username, which is a string type
typedef string Username

// Typedef for a list of user IDs
typedef list<UserId> UserIdList

// Typedef for a map that maps usernames to user IDs
typedef map<Username, UserId> UsernameToUserIdMap

// Typedef for a custom data structure representing a user's address
struct Address {
    1: string street
    2: string city
    3: string state
    4: string zip
}
typedef Address UserAddress

// Typedef for a result type that can be either a user object or an error message
union Result {
    1: User user
    2: string errorMessage
}
typedef Result UserOperationResult

// Typedef for a list of user addresses
typedef list<UserAddress> UserAddressList
