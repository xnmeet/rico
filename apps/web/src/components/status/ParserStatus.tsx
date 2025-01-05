import { cn } from '../../lib/utils';
import {
  Cpu,
  Loader2,
  ShieldCheck,
  AlertOctagon,
  Sparkles,
  CircuitBoard
} from 'lucide-react';

interface ParserStatusProps {
  wasmStatus: {
    isInitializing: boolean;
    error?: string;
  };
  parserStatus: {
    isLoading: boolean;
    error?: string;
  };
  metrics?: {
    parseTime?: number;
    stringifyTime?: number;
  };
  className?: string;
}

export function ParserStatus({
  wasmStatus,
  parserStatus,
  metrics,
  className
}: ParserStatusProps) {
  const getWasmStatus = () => {
    if (wasmStatus.isInitializing) {
      return {
        icon: Loader2,
        color: 'text-amber-500',
        bg: 'bg-amber-50',
        label: 'WASM',
        animate: 'animate-spin'
      };
    }
    if (wasmStatus.error) {
      return {
        icon: AlertOctagon,
        color: 'text-red-500',
        bg: 'bg-red-50',
        label: 'WASM',
        animate: false
      };
    }
    return {
      icon: CircuitBoard,
      color: 'text-emerald-500',
      bg: 'bg-emerald-50',
      label: 'WASM',
      animate: false
    };
  };

  const getParserStatus = () => {
    if (parserStatus.error) {
      return {
        icon: AlertOctagon,
        color: 'text-red-500',
        bg: 'bg-red-50',
        label: 'Parser',
        animate: false
      };
    }
    if (parserStatus.isLoading) {
      return {
        icon: Sparkles,
        color: 'text-blue-500',
        bg: 'bg-blue-50',
        label: 'Parser',
        animate: 'animate-pulse'
      };
    }
    return {
      icon: ShieldCheck,
      color: 'text-emerald-500',
      bg: 'bg-emerald-50',
      label: 'Parser',
      animate: false
    };
  };

  const wasmInfo = getWasmStatus();
  const parserInfo = getParserStatus();
  const WasmIcon = wasmInfo.icon;
  const ParserIcon = parserInfo.icon;

  return (
    <div className={cn('flex items-center gap-2', className)}>
      {/* WASM Status */}
      <div
        className={cn(
          'flex items-center gap-1.5 rounded-full px-2.5 py-1 transition-colors',
          wasmInfo.bg,
          'hover:bg-opacity-100 bg-opacity-80'
        )}>
        <WasmIcon
          className={cn('h-3.5 w-3.5', wasmInfo.color, wasmInfo.animate)}
          strokeWidth={2}
        />
        <span className={cn('text-xs font-medium', wasmInfo.color)}>
          {wasmInfo.label}
        </span>
      </div>

      {/* Parser Status */}
      <div
        className={cn(
          'flex items-center gap-1.5 rounded-full px-2.5 py-1 transition-colors',
          parserInfo.bg,
          'hover:bg-opacity-100 bg-opacity-80'
        )}>
        <ParserIcon
          className={cn('h-3.5 w-3.5', parserInfo.color, parserInfo.animate)}
          strokeWidth={2}
        />
        <span className={cn('text-xs font-medium', parserInfo.color)}>
          {parserInfo.label}
        </span>
      </div>

      {/* Performance Metrics */}
      {metrics && metrics.parseTime && metrics.parseTime > 0 && (
        <div className="flex items-center gap-1.5 rounded-full bg-gray-100/80 px-2.5 py-1 transition-colors hover:bg-gray-100">
          <Cpu className="h-3.5 w-3.5 text-gray-500" />
          <span className="text-xs font-medium text-gray-700">
            {Math.round(metrics.parseTime)}ms
          </span>
        </div>
      )}
    </div>
  );
}
