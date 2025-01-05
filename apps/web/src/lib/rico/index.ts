import Rico from '@rico-core/parser';
import type { Document } from '@rico-core/parser';

export interface ParseResult {
  success: boolean;
  data?: string;
  error?: string;
}

export async function initRicoWasm() {
  try {
    await Rico.initialize();
  } catch (error) {
    console.error('Failed to initialize Rico WASM:', error);
    throw error;
  }
}
