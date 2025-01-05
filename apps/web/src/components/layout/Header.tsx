import { RicoLogo } from '../logo/RicoLogo';
import { ConversionToggle } from '../converter/ConversionToggle';
import { StatusDisplay } from '../status/StatusDisplay';
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
    <header className="relative flex items-center justify-between overflow-hidden rounded-2xl border border-gray-200/20 bg-white/10 px-5 py-3 shadow-[0_8px_16px_-6px_rgb(0_0_0/0.05)] backdrop-blur-xl dark:border-[#363d52]/30 dark:bg-[#282b33]/40 dark:shadow-[0_8px_16px_-6px_rgb(0_0_0/0.2)]">
      <div className="absolute inset-0 bg-gradient-to-r from-white/50 via-white/30 to-white/50 dark:from-[#282b33]/20 dark:via-[#282b33]/15 dark:to-[#282b33]/20" />
      <div className="relative flex items-center gap-3">
        <RicoLogo className="h-8 w-8" />
        <div>
          <h1 className="font-display text-xl font-bold tracking-tight text-gray-900 dark:text-white">
            Rico
          </h1>
          <p className="font-display text-xs font-normal tracking-tight text-gray-500 dark:text-gray-400">
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

      <div className="relative flex items-center gap-3">
        <StatusDisplay
          wasmStatus={wasmStatus}
          parserStatus={parserStatus}
          metrics={parserMetrics}
        />
        <div className="h-4 w-px bg-gray-200/50 dark:bg-white/10" />
        <ThemeToggle />
      </div>
    </header>
  );
}
