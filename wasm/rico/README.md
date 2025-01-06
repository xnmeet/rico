# @rico-core/parser

WebAssembly bindings for Rico - A high-performance Apache Thrift IDL parser and writer library.

## Installation

```bash
npm install @rico-core/parser
```

## Usage

```typescript
import Rico, { Document, Struct } from '@rico-core/parser';

async function main() {
  // Initialize Rico (required before any operations)
  await Rico.initialize();

  const input = `
    namespace rs demo

    struct User {
      1: string name
      2: i32 age
    }
  `;

  try {
    // Parse Thrift IDL to AST with full type information
    const ast: Document = await Rico.parse(input, false);

    // Type-safe access to AST properties
    ast.members.forEach(member => {
      if (member.kind === 'StructDefinition') {
        const struct = member as Struct;
        console.log('Found struct:', struct.name.value);
        struct.members.forEach(field => {
          console.log(
            ` - ${field.name.value}: ${field.fieldType.value} (ID: ${field.fieldID?.value})`
          );
        });
      }
    });

    // Write AST back to Thrift IDL
    const output = Rico.write(ast);
    console.log('Generated Thrift IDL:', output);
  } catch (error) {
    console.error('Error:', error);
  }
}

main().catch(console.error);
```

## API

### `Rico.initialize(): Promise<void>`

Initializes the WebAssembly module. Must be called before using any other methods.

### `Rico.parse(input: string): Promise<Document>`

Parses a Thrift IDL string and returns the AST (Abstract Syntax Tree) with full type information.

### `Rico.write(ast: Document): string`

Converts an AST back to Thrift IDL format.

## Type System

Rico provides a comprehensive type system that exactly matches the Rust AST definitions. All types are automatically generated from the Rust source code to ensure perfect alignment.

### Core Types

- `Document`: The root AST node
- `DocumentMember`: Union type of all possible top-level definitions
- `BaseNode`: Common properties for all AST nodes
- `NodeType`: Enum of all possible node types

### Definition Types

- `Namespace`: Namespace declarations
- `Include`: Include statements
- `Const`: Constant definitions
- `Typedef`: Type aliases
- `Enum`: Enum definitions
- `Struct`: Struct definitions
- `Union`: Union definitions
- `Exception`: Exception definitions
- `Service`: Service definitions

### Field Types

- `Field`: Field definitions in structs/unions/exceptions
- `FieldType`: Union type of all possible field types
  - `CommonType`: Basic types (string, i32, etc.)
  - `FieldSetType`: Sets
  - `FieldListType`: Lists
  - `MapType`: Map types with key and value types

### Value Types

- `FieldValue`: Union type of all possible values
  - `ConstValue`: Basic constant values
  - `ConstList`: List literals
  - `ConstMap`: Map literals

### Location Information

All nodes include source location information:

```typescript
interface LOC {
  start: Span;
  end: Span;
}

interface Span {
  line: number;
  column: number;
  index: number;
}
```

### Type Safety Example

```typescript
import { Document, Struct, Field, FieldType } from '@rico-core/parser';

function processStruct(struct: Struct) {
  // All fields are properly typed
  struct.members.forEach((field: Field) => {
    // Type-safe field access
    const fieldName = field.name.value;
    const fieldType = field.fieldType;
    const isRequired = field.requiredType === 'required';

    // Type-safe pattern matching on field types
    if (fieldType.kind === 'CollectionType') {
      console.log(
        `${fieldName} is a collection of ${fieldType.valueType.value}`
      );
    } else if (fieldType.kind === 'MapType') {
      console.log(
        `${fieldName} is a map from ${fieldType.keyType.value} to ${fieldType.valueType.value}`
      );
    }
  });
}
```

## Development

1. Build the WebAssembly module:

```bash
npm run build
```

2. Run tests:

```bash
npm test
```

## License

MIT
