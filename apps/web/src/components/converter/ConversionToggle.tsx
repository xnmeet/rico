import { Button } from '../ui/button';
import { ArrowLeftRight } from 'lucide-react';
import { cn } from '../../lib/utils';
import { motion, AnimatePresence } from 'framer-motion';

interface ConversionToggleProps {
  isThriftToJson: boolean;
  isDisabled?: boolean;
  onToggle: () => void;
}

export function ConversionToggle({
  isThriftToJson,
  isDisabled,
  onToggle
}: ConversionToggleProps) {
  return (
    <Button
      onClick={onToggle}
      variant="outline"
      className={cn(
        'relative flex items-center gap-2 rounded-full border-gray-200/60 px-4 py-2 text-base shadow-sm transition-colors',
        isDisabled && 'opacity-50 cursor-not-allowed',
        !isDisabled && 'hover:border-gray-300 hover:bg-gray-50'
      )}
      disabled={isDisabled}>
      <div className="relative w-[4.5rem]">
        <AnimatePresence mode="wait">
          <motion.div
            key={isThriftToJson ? 'thrift' : 'json'}
            initial={{
              y: isThriftToJson ? 20 : -20,
              opacity: 0
            }}
            animate={{
              y: 0,
              opacity: 1
            }}
            exit={{
              y: isThriftToJson ? -20 : 20,
              opacity: 0
            }}
            transition={{
              duration: 0.15,
              ease: 'easeInOut'
            }}
            className="absolute inset-0 flex items-center justify-center">
            <span
              className={cn(
                'font-display tracking-tight whitespace-nowrap text-base',
                isThriftToJson ? 'text-blue-600 font-semibold' : 'text-gray-500'
              )}>
              {isThriftToJson ? 'Thrift' : 'JSON'}
            </span>
          </motion.div>
        </AnimatePresence>
      </div>

      <motion.div
        animate={{ rotate: isThriftToJson ? 0 : 180 }}
        transition={{
          type: 'spring',
          stiffness: 200,
          damping: 30
        }}
        className="mx-0.5">
        <ArrowLeftRight className="h-4 w-4 text-gray-400" />
      </motion.div>

      <div className="relative w-[4.5rem]">
        <AnimatePresence mode="wait">
          <motion.div
            key={isThriftToJson ? 'json' : 'thrift'}
            initial={{
              y: isThriftToJson ? -20 : 20,
              opacity: 0
            }}
            animate={{
              y: 0,
              opacity: 1
            }}
            exit={{
              y: isThriftToJson ? 20 : -20,
              opacity: 0
            }}
            transition={{
              duration: 0.15,
              ease: 'easeInOut'
            }}
            className="absolute inset-0 flex items-center justify-center">
            <span
              className={cn(
                'font-display tracking-tight whitespace-nowrap text-base',
                !isThriftToJson
                  ? 'text-blue-600 font-semibold'
                  : 'text-gray-500'
              )}>
              {isThriftToJson ? 'JSON' : 'Thrift'}
            </span>
          </motion.div>
        </AnimatePresence>
      </div>
    </Button>
  );
}
