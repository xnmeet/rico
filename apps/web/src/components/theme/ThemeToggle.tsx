import React from 'react';
import { Button } from '../ui/button';
import { Sun, Moon } from 'lucide-react';
import { useTheme } from './theme-provider';
import { cn } from '../../lib/utils';

export function ThemeToggle() {
  const { theme, setTheme } = useTheme();

  return (
    <Button
      variant="ghost"
      size="icon"
      className={cn(
        'h-8 w-8 rounded-full',
        'text-gray-500 hover:text-gray-900 dark:text-gray-400 dark:hover:text-gray-100'
      )}
      onClick={() => setTheme(theme === 'dark' ? 'light' : 'dark')}>
      <Sun className="h-4 w-4 rotate-0 scale-100 transition-transform duration-200 dark:-rotate-90 dark:scale-0" />
      <Moon className="absolute h-4 w-4 rotate-90 scale-0 transition-transform duration-200 dark:rotate-0 dark:scale-100" />
      <span className="sr-only">Toggle theme</span>
    </Button>
  );
}
