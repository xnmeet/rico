import Rico, { Document } from '../src';

async function main() {
  // Initialize Rico
  await Rico.initialize();

  const input = `
    namespace rs demo

    struct User {
      1: string name
      2: i32 age
      3: map<string, list<i32>> getUserStatistics()
    }
  `;

  try {
    // Parse Thrift IDL to AST with proper typing
    const ast = await Rico.parse(input, false);
    console.log('Parsed AST:', JSON.stringify(ast, null, 2));

    // Type-safe access to AST properties
    ast.members.forEach(member => {
      if (member.kind === 'StructDefinition') {
        console.log('Found struct:', member.name.value);
        member.members.forEach(field => {
          console.log(
            ` - ${field.name.value}: ${field.fieldType.value} (ID: ${field.fieldID?.value})`
          );
        });
      }
    });

    // Write AST back to Thrift IDL
    const output = Rico.write(ast);
    console.log('Generated Thrift IDL:\n', output);
  } catch (error) {
    console.error('Error:', error);
  }
}

main().catch(console.error);
