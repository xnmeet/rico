import Rico from '@rico-core/parser';
import { type ParseResult } from '../rico';

export class ParserService {
  private static instance: ParserService;

  private constructor() {}

  public static getInstance(): ParserService {
    if (!ParserService.instance) {
      ParserService.instance = new ParserService();
    }
    return ParserService.instance;
  }

  async parse(content: string): Promise<ParseResult> {
    if (!content.trim()) {
      return {
        success: false,
        error: 'Please enter some Thrift IDL content'
      };
    }

    try {
      const result = await Rico.parse(content);
      return {
        success: true,
        data: result
      };
    } catch (error) {
      return {
        success: false,
        error: error instanceof Error ? error.message : String(error)
      };
    }
  }

  async generate(content: string): Promise<ParseResult> {
    if (!content.trim()) {
      return {
        success: false,
        error: 'Please enter some JSON content'
      };
    }

    try {
      // Parse JSON first
      const jsonData = JSON.parse(content);
      const result = Rico.write(jsonData);
      return {
        success: true,
        data: result
      };
    } catch (error) {
      if (error instanceof SyntaxError) {
        return {
          success: false,
          error: 'Invalid JSON format: ' + error.message
        };
      }
      return {
        success: false,
        error: error instanceof Error ? error.message : String(error)
      };
    }
  }
}
