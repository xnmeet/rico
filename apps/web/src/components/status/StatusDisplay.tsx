import { Activity, Loader2, Cpu } from 'lucide-react';
import type {
  WasmStatus,
  ParserStatus,
  ParserMetrics
} from '../../lib/constants';

interface StatusDisplayProps {
  wasmStatus: WasmStatus;
  parserStatus: ParserStatus;
  metrics: ParserMetrics;
}

export function StatusDisplay({
  wasmStatus,
  parserStatus,
  metrics
}: StatusDisplayProps) {
  const isLoading = parserStatus.isLoading;
  const hasError = parserStatus.error;
  const parseTime = metrics.parseTime ?? 0;
  const showParserStatus = isLoading || hasError || parseTime > 0;

  return (
    <div className="w-[180px] flex items-center justify-end gap-2 text-xs">
      <div className="flex items-center gap-2">
        <div className="flex items-center gap-1.5 rounded-full bg-gray-900/[0.015] px-2.5 py-1 dark:bg-gray-300/[0.03]">
          <Cpu
            className={`h-3.5 w-3.5 ${
              wasmStatus.error
                ? 'text-red-500'
                : wasmStatus.isInitializing
                ? 'animate-pulse text-blue-500'
                : 'text-emerald-500'
            }`}
          />
          <span
            className={`font-medium tracking-tight ${
              wasmStatus.error
                ? 'text-red-600 dark:text-red-400'
                : wasmStatus.isInitializing
                ? 'text-blue-600 dark:text-blue-400'
                : 'text-emerald-600 dark:text-emerald-400'
            }`}>
            {wasmStatus.error
              ? 'Error'
              : wasmStatus.isInitializing
              ? 'Loading'
              : 'Ready'}
          </span>
        </div>

        {showParserStatus && (
          <div className="flex items-center gap-1.5 rounded-full bg-gray-900/[0.015] px-2.5 py-1 dark:bg-gray-300/[0.03]">
            {isLoading ? (
              <Loader2 className="h-3.5 w-3.5 animate-spin text-yellow-500" />
            ) : hasError ? (
              <Activity className="h-3.5 w-3.5 text-red-500" />
            ) : (
              <Activity className="h-3.5 w-3.5 text-emerald-500" />
            )}
            <div className="font-medium tracking-tight">
              {hasError ? (
                <span className="text-red-600 dark:text-red-400">Error</span>
              ) : isLoading ? (
                <span className="text-yellow-600 dark:text-yellow-400">
                  Processing
                </span>
              ) : (
                <span className="text-gray-600 dark:text-gray-300">
                  {parseTime.toFixed(1)}ms
                </span>
              )}
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
