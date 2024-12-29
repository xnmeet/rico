use rico::ast::*;

#[test]
fn test_ast_creation() {
    let span = Span::new(1, 1, 0);
    let loc = LOC {
        start: span,
        end: span,
    };

    let common = Common::new(NodeType::Identifier, "test".to_string(), loc);

    assert_eq!(common.kind, NodeType::Identifier);
    assert_eq!(common.value, "test");
}
