import { RicoLogo } from '../logo/RicoLogo';
import { ConversionToggle } from '../converter/ConversionToggle';
import { ParserStatus } from '../status/ParserStatus';
import { ThemeToggle } from '../theme/ThemeToggle';
import type {
  WasmStatus,
  ParserStatus as ParserStatusType,
  ParserMetrics
} from '../../lib/constants';

interface HeaderProps {
  isThriftToJson: boolean;
  isDisabled: boolean;
  onToggleMode: () => void;
  wasmStatus: WasmStatus;
  parserStatus: ParserStatusType;
  parserMetrics: ParserMetrics;
}

export function Header({
  isThriftToJson,
  isDisabled,
  onToggleMode,
  wasmStatus,
  parserStatus,
  parserMetrics
}: HeaderProps) {
  return (
    <header className="relative flex items-center justify-between overflow-hidden rounded-2xl border border-gray-200/20 bg-white/10 px-6 py-4 shadow-[0_8px_16px_-6px_rgb(0_0_0/0.05)] backdrop-blur-xl dark:border-white/10 dark:bg-gray-800/10 dark:shadow-[0_8px_16px_-6px_rgb(0_0_0/0.15)]">
      <div className="absolute inset-0 bg-gradient-to-r from-white/50 via-white/30 to-white/50 dark:from-white/[0.03] dark:via-white/[0.02] dark:to-white/[0.03]" />
      <div className="relative flex items-center gap-3">
        <RicoLogo />
        <div>
          <h1 className="font-display text-2xl font-bold tracking-tight text-gray-900 dark:text-white">
            Rico
            <span className="font-display font-medium text-gray-600 dark:text-gray-400">
              {' '}
              Thrift Converter
            </span>
          </h1>
          <p className="font-display text-sm font-medium tracking-tight text-gray-500 dark:text-gray-400">
            Convert between Thrift IDL and JSON with ease
          </p>
        </div>
      </div>

      <div className="relative z-10">
        <ConversionToggle
          isThriftToJson={isThriftToJson}
          isDisabled={isDisabled}
          onToggle={onToggleMode}
        />
      </div>

      <div className="relative flex items-center space-x-4">
        <ParserStatus
          wasmStatus={wasmStatus}
          parserStatus={parserStatus}
          metrics={parserMetrics}
        />
        <div className="ml-2 h-4 w-px bg-gray-200/50 dark:bg-white/10" />
        <ThemeToggle />
      </div>
    </header>
  );
}
