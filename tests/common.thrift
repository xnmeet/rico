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

// 下面是一些不规范的写法示例

// 不一致的缩进
struct InconsistentIndentation {
  1: i32 id,
    2: string name, // 这个字段的缩进不一致
  3: string email
}

// 重复的字段名
struct DuplicateFieldNames {
  1: string name,
  2: string name, // 这个字段名重复了
  3: i32 age
}

// 缺少必要的字段
struct IncompleteUser {
  1: i32 id,
  // 2: string name, // 这个字段缺失
  3: string email
}

// 不规范的注释风格
// This is a comment without a clear purpose
// 这行注释没有提供有用的信息
struct UnclearComment {
  1: string data // 这个字段的注释不清晰
}

// 额外的空行
struct ExtraEmptyLines {

  1: string info


  // 这个结构体中有多余的空行
}

// 过长的行
struct LongLineExample {
  1: string description = "This is a very long line that exceeds the typical length for a single line in Thrift IDL and should be broken up for better readability."
}

// 过多的空格
struct ExcessiveSpaces {
  1: string   name, // 这个字段前有过多的空格
  2: i32 age,
  3: list<string> hobbies = [
    # 另一种不规范的写法
    "reading", // 值注释
     "traveling", "coding"]
}


