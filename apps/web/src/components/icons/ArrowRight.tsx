import { cn } from '../../lib/utils';

interface ArrowRightProps {
  className?: string;
}

export function ArrowRight({ className }: ArrowRightProps) {
  return (
    <svg
      xmlns="http://www.w3.org/2000/svg"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth="2"
      strokeLinecap="round"
      strokeLinejoin="round"
      className={cn('w-6 h-6', className)}>
      <path d="M5 12h14m-7-7 7 7-7 7" />
    </svg>
  );
}
