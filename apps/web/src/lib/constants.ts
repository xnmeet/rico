export const CONVERSION_MODES = {
  THRIFT_TO_JSON: 'thrift-to-json',
  JSON_TO_THRIFT: 'json-to-thrift'
} as const;

export type ConversionMode =
  (typeof CONVERSION_MODES)[keyof typeof CONVERSION_MODES];

export interface WasmStatus {
  isInitializing: boolean;
  error?: string;
}

export interface ParserStatus {
  isLoading: boolean;
  error?: string;
}

export interface ParserMetrics {
  parseTime?: number;
  stringifyTime?: number;
}
