import init, { Parser, Writer } from './wasm/rico_wasm';
import type { Document, ParseError } from './types';

export class RicoError extends Error {
  constructor(public details: ParseError) {
    const message = [
      `${details.message}${
        details.location &&
        `(${details.location.line}:${details.location.column})`
      }`,
      details.help && `Help: ${details.help}`
    ]
      .filter(Boolean)
      .join('\n');

    super(message);
    this.name = 'RicoError';
    // Prevent V8 from collecting stack trace
    Error.captureStackTrace(this, RicoError);
  }
}

export class Rico {
  private static initialized = false;

  static async initialize(): Promise<void> {
    if (!Rico.initialized) {
      await init();
      Rico.initialized = true;
    }
  }

  static parse(input: string): Promise<string>;
  static parse(input: string, toString: false): Promise<Document>;
  static parse(input: string, toString: true): Promise<string>;
  static parse(
    input: string,
    toString: boolean = true
  ): Promise<Document | string> {
    if (!Rico.initialized) {
      throw new Error('Rico is not initialized. Call Rico.initialize() first.');
    }
    const parser = new Parser(input);
    try {
      const result = parser.parse();
      return Promise.resolve(!toString ? JSON.parse(result) : result);
    } catch (error) {
      try {
        const parserError = JSON.parse(error as string);
        if (parserError && 'kind' in parserError) {
          throw new RicoError(error as ParseError);
        }
      } finally {
        throw error;
      }
    }
  }

  static write(ast: Document): string {
    if (!Rico.initialized) {
      throw new Error('Rico is not initialized. Call Rico.initialize() first.');
    }
    const writer = new Writer();
    try {
      return writer.write(JSON.stringify(ast));
    } catch (error) {
      try {
        const parserError = JSON.parse(error as string);
        if (parserError && 'kind' in parserError) {
          throw new RicoError(error as ParseError);
        }
      } finally {
        throw error;
      }
    }
  }
}

export * from './types';
export default Rico;
