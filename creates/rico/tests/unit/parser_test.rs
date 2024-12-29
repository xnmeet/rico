
    use rico::DocumentMembers;
    use rico::FieldInitialValue;
    use rico::FieldType;
    use rico::Parser;

    #[test]
    fn test_parse_namespace() {
        let input = "namespace rs demo";
        let mut parser = Parser::new(input);
        let result = parser.parse().unwrap();

        match &result.members[0] {
            DocumentMembers::Namespace(ns) => {
                assert_eq!(ns.scope.value, "rs");
                assert_eq!(ns.name.value, "demo");
            }
            _ => panic!("Expected Namespace"),
        }
    }

    #[test]
    fn test_parse_struct() {
        let input = r#"
            struct User {
                1: string name
                2: i32 age
                3: optional list<string> tags
            }
        "#;
        let mut parser = Parser::new(input);
        let result = parser.parse().unwrap();

        match &result.members[0] {
            DocumentMembers::Struct(s) => {
                assert_eq!(s.name.value, "User");
                assert_eq!(s.members.len(), 3);
                assert_eq!(s.members[0].name.value, "name");
                assert_eq!(s.members[1].name.value, "age");
                assert_eq!(s.members[2].name.value, "tags");
            }
            _ => panic!("Expected Struct"),
        }
    }

    #[test]
    fn test_parse_enum() {
        let input = r#"
            enum Status {
                ACTIVE = 1
                INACTIVE = 2
                DELETED = 3 (deprecated = "use INACTIVE instead")
            }
        "#;
        let mut parser = Parser::new(input);
        let result = parser.parse().unwrap();

        match &result.members[0] {
            DocumentMembers::Enum(e) => {
                assert_eq!(e.name.value, "Status");
                assert_eq!(e.members.len(), 3);
                assert_eq!(e.members[0].name.value, "ACTIVE");
                assert_eq!(e.members[1].name.value, "INACTIVE");
                assert_eq!(e.members[2].name.value, "DELETED");
            }
            _ => panic!("Expected Enum"),
        }
    }

    #[test]
    fn test_parse_service() {
        let input = r#"
            service UserService {
                User getUser(1: i32 id) throws (1: UserNotFound notFound)
                void createUser(1: User user)
                oneway void notify(1: string message)
            }
        "#;
        let mut parser = Parser::new(input);
        let result = parser.parse().unwrap();

        match &result.members[0] {
            DocumentMembers::Service(s) => {
                assert_eq!(s.name.value, "UserService");
                assert_eq!(s.members.len(), 3);
                assert_eq!(s.members[0].name.value, "getUser");
                assert_eq!(s.members[1].name.value, "createUser");
                assert_eq!(s.members[2].name.value, "notify");
                assert!(s.members[2].oneway);
            }
            _ => panic!("Expected Service"),
        }
    }

    #[test]
    fn test_parse_typedef() {
        let input = "typedef i64 UserId";
        let mut parser = Parser::new(input);
        let result = parser.parse().unwrap();

        match &result.members[0] {
            DocumentMembers::Typedef(t) => {
                assert_eq!(t.name.value, "UserId");
                match &t.field_type {
                    FieldType::CommonType(c) => assert_eq!(c.value, "i64"),
                    _ => panic!("Expected CommonType"),
                }
            }
            _ => panic!("Expected Typedef"),
        }
    }

    #[test]
    fn test_parse_const() {
        let input = r#"
            const i32 MAX_USERS = 1000
            const string VERSION = "1.0.0"
            const list<string> ADMINS = ["admin", "root"]
        "#;
        let mut parser = Parser::new(input);
        let result = parser.parse().unwrap();

        assert_eq!(result.members.len(), 3);
        match &result.members[0] {
            DocumentMembers::Const(c) => {
                assert_eq!(c.name.value, "MAX_USERS");
                match &c.value {
                    FieldInitialValue::ConstValue(v) => assert_eq!(v.value, "1000"),
                    _ => panic!("Expected ConstValue"),
                }
            }
            _ => panic!("Expected Const"),
        }
    }

    #[test]
    fn test_parse_comments() {
        let input = r#"
            // Service comment
            /* Block comment */
            service UserService {
                // Method comment
                void createUser(1: User user)
            }
        "#;
        let mut parser = Parser::new(input);
        let result = parser.parse().unwrap();

        match &result.members[0] {
            DocumentMembers::Service(s) => {
                assert_eq!(s.comments.len(), 2);
                assert_eq!(s.members[0].comments.len(), 1);
            }
            _ => panic!("Expected Service"),
        }
    }

    #[test]
    fn test_parse_annotations() {
        let input = r#"
            struct User {
                1: string name (max_length = "100")
                2: i32 age (min = "0", max = "150")
            }
        "#;
        let mut parser = Parser::new(input);
        let result = parser.parse().unwrap();

        match &result.members[0] {
            DocumentMembers::Struct(s) => {
                assert!(s.members[0].annotations.is_some());
                assert!(s.members[1].annotations.is_some());
            }
            _ => panic!("Expected Struct"),
        }
    }

    #[test]
    fn test_parse_errors() {
        let invalid_inputs = vec![
            "struct {}",                       // Missing struct name
            "enum Status { ACTIVE = }",        // Invalid enum value
            "const string NAME",               // Missing const value
            "service UserService { invalid }", // Invalid service method
        ];

        for input in invalid_inputs {
            let mut parser = Parser::new(input);
            assert!(parser.parse().is_err());
        }
    }

