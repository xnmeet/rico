import { RefreshCw } from 'lucide-react';
import { motion, AnimatePresence } from 'framer-motion';

interface ConversionToggleProps {
  isThriftToJson: boolean;
  isDisabled: boolean;
  onToggle: () => void;
}

export function ConversionToggle({
  isThriftToJson,
  isDisabled,
  onToggle
}: ConversionToggleProps) {
  return (
    <button
      onClick={onToggle}
      disabled={isDisabled}
      className="group relative flex h-8 items-center rounded-lg bg-white/80 px-1 shadow-sm ring-1 ring-gray-200/50 transition hover:bg-white/90 disabled:opacity-50 dark:bg-[#1a1b26]/90 dark:ring-[#2c2e3f] dark:hover:bg-[#1a1b26]">
      <div className="relative flex items-center gap-3 px-3">
        <div className="w-14 text-center">
          <AnimatePresence mode="wait">
            <motion.div
              key={isThriftToJson ? 'thrift' : 'json'}
              initial={{ y: 10, opacity: 0 }}
              animate={{ y: 0, opacity: 1 }}
              exit={{ y: -10, opacity: 0 }}
              transition={{ duration: 0.15 }}
              className="text-gray-900 dark:text-[#c0caf5]">
              <span className="text-xs font-semibold tracking-wide">
                {isThriftToJson ? 'THRIFT' : 'JSON'}
              </span>
            </motion.div>
          </AnimatePresence>
        </div>

        <div className="relative flex h-5 w-5 items-center justify-center">
          <div className="absolute inset-0 rounded-full bg-gray-900/[0.03] shadow-sm ring-1 ring-gray-900/[0.05] transition group-hover:bg-gray-900/[0.05] dark:bg-[#2c2e3f] dark:ring-[#414868] dark:group-hover:bg-[#414868]" />
          <motion.div
            animate={{ rotate: isThriftToJson ? 360 : 0 }}
            transition={{
              type: 'spring',
              stiffness: 200,
              damping: 10
            }}>
            <RefreshCw className="relative h-3 w-3 text-gray-500 transition-colors group-hover:text-gray-900 dark:text-[#7aa2f7] dark:group-hover:text-[#89b4f7]" />
          </motion.div>
        </div>

        <div className="w-14 text-center">
          <AnimatePresence mode="wait">
            <motion.div
              key={isThriftToJson ? 'json' : 'thrift'}
              initial={{ y: -10, opacity: 0 }}
              animate={{ y: 0, opacity: 1 }}
              exit={{ y: 10, opacity: 0 }}
              transition={{ duration: 0.15 }}
              className="text-gray-400 dark:text-[#565f89]">
              <span className="text-xs font-semibold tracking-wide">
                {isThriftToJson ? 'JSON' : 'THRIFT'}
              </span>
            </motion.div>
          </AnimatePresence>
        </div>
      </div>
    </button>
  );
}
