// help me generate thrift code according to Apache Thrift IDL syntax

namespace py mock_example
namespace cpp mock_example_cpp

// This is a multi-line comment
// It can span multiple lines
// and is useful for providing detailed explanations

// Include another Thrift file
include "common.thrift"

// Enum definition for user roles
enum UserRole {
  ADMIN = 1,
  USER = 2,
  GUEST = 3
}

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

// Union type for a value that can be either an integer or a string
union IntOrString {
    1: i32 int_value
    2: string string_value
}

// Union type for a result that can be a user object, an error message, or a boolean indicating success
union UserResult {
    1: User user_data
    2: string error_message
    3: bool success
}

// Union type for a configuration option that can be an integer, a string, or a list of strings
union ConfigOption {
    1: i32 int_setting
    2: string string_setting
    3: list<string> string_list_setting
}

// Union type for a data container that can hold different types of data related to an item
union ItemData {
    1: i32 item_id
    2: string item_name
    3: map<string, i32> item_attributes
    4: list<ItemDetail> item_details
}

// Constant of integer type representing the default port number
const i32 DEFAULT_PORT = 8080

// Constant of string type for the default hostname
const string DEFAULT_HOSTNAME = "localhost"

// Constant of boolean type indicating if the application is in debug mode
const bool IS_DEBUG_MODE = false

// Constant of list<string> type with some default usernames
const list<string> DEFAULT_USERNAMES = ["user1", "user2", "user3"]

// Constant of map<string, i32> type mapping department names to their respective ID numbers
const map<string, i32> DEPARTMENT_IDS = {
    "Engineering": 1001,
    "Marketing": 1002,
    "Sales": 1003
}
// Struct definition for User
struct User {
  1: i32 id,
  2: string name,
  3: string email,
  4: UserStatus status,
  5: UserRole role
}

// Nested struct definition for Address
struct Address {
  1: string street,
  2: string city,
  3: string state,
  4: string zip
}

// Struct with default values for UserProfile
struct UserProfile {
  1: User user,
  2: Address address,
  3: list<string> hobbies = [],
  4: map<string, string> preferences = {}
}

// Exception definition for user not found
exception UserNotFoundException {
  1: string message
}

// Service definition for UserService
service UserService {
  // Method to get a user by ID
  User getUser(1: i32 id) throws (1: UserNotFoundException e),

  // Method to save a user
  void saveUser(1: User user),

  // Method to delete a user
  void deleteUser(1: i32 id) throws (1: UserNotFoundException e),

  // Method to list all users
  list<User> listUsers(),

  // Method to update user profile
  void updateUserProfile(1: UserProfile profile) throws (1: UserNotFoundException e)
}

// Service definition for AdminService
service AdminService {
  // Method to suspend a user
  void suspendUser(1: i32 id) throws (1: UserNotFoundException e),

  // Method to activate a user
  void activateUser(1: i32 id) throws (1: UserNotFoundException e),

  // Method to get user statistics
  map<string, i32> getUserStatistics()
}

// Service definition for NotificationService
service NotificationService {
  // Method to send a notification
  void sendNotification(1: string message, 2: list<i32> userIds),

  // Method to get notifications for a user
  list<string> getUserNotifications(1: i32 userId)
}

// The following are examples of irregular writing

// Inconsistent indentation
struct InconsistentIndentation {
  1: i32 id,
    2: string name, // The indentation of this field is inconsistent
  3: string email
}

// Duplicate field names
struct DuplicateFieldNames {
  1: string name,
  2: string name, // This field name is duplicated
  3: i32 age
}

// Missing necessary fields
struct IncompleteUser {
  1: i32 id,
  // 2: string name, // This field is missing
  3: string email
}

// Irregular comment style
// This is a comment without a clear purpose
// This line of comment doesn't provide useful information
struct UnclearComment {
  1: string data // The comment for this field is not clear
}

// Extra blank lines
struct ExtraEmptyLines {

  1: string info


  // There are redundant blank lines in this struct
}

// Long line
struct LongLineExample {
  1: string description = "This is a very long line that exceeds the typical length for a single line in Thrift IDL and should be broken up for better readability."
}

// Excessive spaces
struct ExcessiveSpaces {
  1: string   name, // There are excessive spaces before this field
  2: i32 age,
  3: list<string> hobbies = [
    # Another irregular writing
    "reading", // Value comment
     "traveling", "coding"]
}