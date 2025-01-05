export const EXAMPLE_THRIFT = `namespace js example

struct User {
  1: required string id
  2: required string name
  3: optional i32 age
}

service UserService {
  User getUser(1: string id)
  void createUser(1: User user)
}`;

export const EXAMPLE_JSON = `{
  "kind": "ThriftDocument",
  "members": [
    {
      "kind": "NamespaceDefinition",
      "loc": {
        "start": {
          "line": 1,
          "column": 1,
          "index": 0
        },
        "end": {
          "line": 1,
          "column": 21,
          "index": 20
        }
      },
      "name": {
        "kind": "Identifier",
        "value": "example",
        "loc": {
          "start": {
            "line": 1,
            "column": 14,
            "index": 13
          },
          "end": {
            "line": 1,
            "column": 21,
            "index": 20
          }
        }
      },
      "scope": {
        "kind": "Identifier",
        "value": "js",
        "loc": {
          "start": {
            "line": 1,
            "column": 11,
            "index": 10
          },
          "end": {
            "line": 1,
            "column": 13,
            "index": 12
          }
        }
      },
      "comments": []
    }
  ]
}`;
