import { cn } from '../../lib/utils';

interface RicoLogoProps {
  className?: string;
}

export function RicoLogo({ className }: RicoLogoProps) {
  return (
    <div className={cn('relative flex items-center', className)}>
      {/* Logo Container */}
      <div className="relative h-8 w-8">
        {/* Background Shapes */}
        <div className="absolute inset-0 rounded-lg bg-gradient-to-br from-gray-800 to-gray-900 shadow-lg" />
        <div className="absolute -inset-0.5 -rotate-6 rounded-lg bg-gradient-to-br from-gray-700 to-gray-800 opacity-75" />

        {/* Accent Line */}
        <div className="absolute bottom-1 left-1 right-1 h-0.5 rounded-full bg-gradient-to-r from-blue-500/40 via-blue-400/60 to-blue-500/40" />

        {/* Letter R */}
        <div className="absolute inset-0 flex items-center justify-center">
          <span
            className="font-display text-lg font-bold tracking-tight text-gray-100"
            style={{ transform: 'translateY(-1px)' }}>
            R
          </span>
        </div>

        {/* Decorative Elements */}
        <div className="absolute -right-0.5 -top-0.5 h-1.5 w-1.5 rounded-full bg-blue-400/60" />
        <div className="absolute -bottom-0.5 -left-0.5 h-1 w-1 rounded-full bg-blue-500/40" />
      </div>
    </div>
  );
}
