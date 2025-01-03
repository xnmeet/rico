import init, { Parser, Writer } from './wasm/rico_wasm';
import type { Document } from './types';

export class Rico {
  private static initialized = false;

  static async initialize(): Promise<void> {
    if (!Rico.initialized) {
      await init();
      Rico.initialized = true;
    }
  }

  static parse(input: string): Promise<Document> {
    if (!Rico.initialized) {
      throw new Error('Rico is not initialized. Call Rico.initialize() first.');
    }
    const parser = new Parser(input);
    return parser.parse() as Promise<Document>;
  }

  static write(ast: Document): string {
    if (!Rico.initialized) {
      throw new Error('Rico is not initialized. Call Rico.initialize() first.');
    }
    const writer = new Writer();
    return writer.write(ast);
  }
}

export * from './types';
export default Rico;
