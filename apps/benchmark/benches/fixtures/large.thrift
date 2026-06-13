// Rico benchmark fixture: broad Apache Thrift IDL syntax coverage.
/* Header block comment spanning
 * multiple lines to verify location tracking.
 */
cpp_include "generated/rico_types.h";
include "shared/common.thrift";

namespace * rico.fixtures;
namespace rs rico.fixtures.rs
namespace cpp2 rico.fixtures.cpp2
namespace py rico.fixtures.py

# Type aliases
typedef i64 UserId;
typedef map<string, list<i32>> ScoreBuckets
typedef set<string> StringSet

// Primitive constants
const bool CONST_BOOL_TRUE = true
const bool CONST_BOOL_FALSE = false
const byte CONST_BYTE = 7
const i8 CONST_I8 = -8
const i16 CONST_I16 = -16
const i32 CONST_I32 = 2147483647
const i64 CONST_I64 = 9223372036854775807
const double CONST_DOUBLE = -42.125
const double CONST_EXP = 6.02e23
const double CONST_FRACTION = .5
const string CONST_STRING = "escaped \"quote\" and slash \\"
const binary CONST_BINARY = 'raw-bytes'
const i32 CONST_HEX = 0x2A
const i32 CONST_REF = CONST_I32

const list<string> CONST_LIST = [
  // Value comments should not become comments on later definitions.
  "alpha",
  # Hash comments inside values are trivia.
  "beta";
  /* Block comments inside values are trivia. */
  "gamma",
]

const map<string, list<i32>> CONST_MAP = {
  "one": [1, 2, 3],
  // Comment between map properties.
  "two": [
    4;
    5,
    6,
  ],
}

enum Lifecycle {
  UNKNOWN = 0,
  ACTIVE = 1 (description = "currently active"),
  DISABLED = 2;
  DELETED = 0x03
} (
  // Annotation comments are trivia.
  rust.exhaustive = "false";
  owner = "rico"
);

enum SignedEnum {
  NEGATIVE = -1
  ZERO = 0
  POSITIVE = 1
}

struct EmptyStruct {
};

// All primitive and container field forms.
struct PrimitiveBag {
  1: required bool bool_value = true (js.name = "boolValue"),
  2: optional byte byte_value = 1,
  3: optional i8 i8_value = -8,
  4: optional i16 i16_value = -16,
  5: optional i32 i32_value = 32,
  6: optional i64 i64_value = 64,
  7: optional double double_value = 1.25,
  8: optional string string_value = "hello",
  9: optional binary binary_value = "bytes",
  10: optional list<string> list_value = [
    "first",
    "second",
  ],
  11: optional set<i32> set_value,
  12: optional map<string, list<set<i32>>> nested_value,
  13: Lifecycle lifecycle = ACTIVE,
  // Leading comment for the next field must stay attached to that field.
  14: optional string commented_field,
} (table = "primitive_bag")

struct CommentStress {
  1: list<string> values = [
    "first",
    // This value comment must be discarded.
    "second",
  ],
  2: string after_values,
  # Hash comment before field three.
  3: optional map<string, string> metadata = {
    "left": "right",
    /* Block value comment must be discarded. */
    "up": "down",
  },
};

union SearchKey {
  1: UserId user_id,
  2: string email,
  3: binary digest,
} (serde = "untagged")

exception ValidationError {
  1: required string message,
  2: optional map<string, string> details,
} (retryable = "false");

service BaseService {
  void ping();
}

service UserService extends BaseService {
  // Fetch with comments inside the parameter list.
  PrimitiveBag get_user(
    1: required UserId id,
    // Parameter leading comment.
    2: optional string request_id,
  ) throws (
    1: ValidationError validation_error,
  ) (route = "/users/{id}")

  oneway void publish_event(1: string topic, 2: binary payload)

  map<string, list<PrimitiveBag>> search(
    1: SearchKey key;
    2: ScoreBuckets scores,
    3: StringSet tags = [],
  )
} (service.version = "1")
