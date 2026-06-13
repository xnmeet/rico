import init, {
  parse as parseWasm,
  write as writeWasm
} from './wasm/rico_wasm';
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
    if ('captureStackTrace' in Error) {
      Error.captureStackTrace(this, RicoError);
    }
  }
}

export class Rico {
  private static initialized = false;

  static async initialize(moduleOrPath?: Parameters<typeof init>[0]): Promise<void> {
    if (!Rico.initialized) {
      await init(moduleOrPath);
      Rico.initialized = true;
    }
  }

  static parse(input: string): string;
  static parse(input: string, toString: false): Document;
  static parse(input: string, toString: true): string;
  static parse(input: string, toString?: boolean): Document | string {
    if (!Rico.initialized) {
      throw new Error('Rico is not initialized. Call Rico.initialize() first.');
    }
    toString = typeof toString === 'undefined' ? true : toString;
    try {
      const result = parseWasm(input);
      return !toString ? JSON.parse(result) : result;
    } catch (error) {
      throw toRicoError(error);
    }
  }

  static parseObject(input: string): Document {
    return Rico.parse(input, false);
  }

  static parseString(input: string): string {
    return Rico.parse(input, true);
  }

  static write(ast: Document): string {
    if (!Rico.initialized) {
      throw new Error('Rico is not initialized. Call Rico.initialize() first.');
    }
    try {
      return writeWasm(JSON.stringify(ast));
    } catch (error) {
      throw toRicoError(error);
    }
  }
}

function toRicoError(error: unknown): unknown {
  if (typeof error !== 'string') {
    return error;
  }

  try {
    const parserError = JSON.parse(error) as ParseError;
    if (parserError && 'kind' in parserError) {
      return new RicoError(parserError);
    }
  } catch {
    // Fall through to the original WASM error string.
  }

  return error;
}

export const initialize = Rico.initialize.bind(Rico);
export const parse = Rico.parse.bind(Rico);
export const parseObject = Rico.parseObject.bind(Rico);
export const parseString = Rico.parseString.bind(Rico);
export const write = Rico.write.bind(Rico);
export * from './types';
export default Rico;
