import React, { useRef } from 'react';
import CodeMirror, { ReactCodeMirrorRef } from '@uiw/react-codemirror';
import { githubLight, githubDark } from '@uiw/codemirror-theme-github';
import { json } from '@codemirror/lang-json';
import { cn } from '../../lib/utils';
import { Copy, Check, Braces, Code2 } from 'lucide-react';
import { Button } from '../ui/button';
import { useTheme } from '../theme/theme-provider';

interface ThriftEditorProps {
  value: string;
  onChange?: (value: string) => void;
  editable?: boolean;
  title: string;
  subtitle?: string;
  className?: string;
  language?: 'thrift' | 'json';
  ref?: React.Ref<HTMLDivElement>;
}

export function ThriftEditor({
  value,
  onChange,
  editable = true,
  title,
  subtitle,
  className,
  language = 'thrift',
  ref
}: ThriftEditorProps) {
  const [copied, setCopied] = React.useState(false);
  const editorRef = useRef<ReactCodeMirrorRef>(null);
  const { theme } = useTheme();

  const handleCopy = async () => {
    try {
      await navigator.clipboard.writeText(value);
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    } catch (err) {
      console.error('Failed to copy text:', err);
    }
  };

  const handleChange = (value: string) => {
    if (!editable) {
      return;
    }
    onChange?.(value);
  };

  const TitleIcon = language === 'json' ? Braces : Code2;

  return (
    <div
      ref={ref}
      className={cn(
        'flex h-full flex-col overflow-hidden rounded-lg bg-white shadow-sm dark:bg-[#2b3245]/90',
        className
      )}>
      <div className="flex items-center justify-between border-b border-gray-100 px-4 py-2 dark:border-[#363d52]/50">
        <div className="flex items-center gap-2">
          <TitleIcon className="h-4 w-4 text-blue-500/70" strokeWidth={2.5} />
          <div>
            <h2 className="font-medium text-gray-700 dark:text-gray-200">
              {title}
            </h2>
            {subtitle && (
              <p className="text-xs text-gray-500 dark:text-gray-400">
                {subtitle}
              </p>
            )}
          </div>
        </div>
        <Button
          variant="ghost"
          size="icon"
          className="h-8 w-8 text-gray-500 hover:text-gray-900 dark:text-gray-400 dark:hover:text-gray-100"
          onClick={handleCopy}>
          {copied ? (
            <Check className="h-4 w-4" />
          ) : (
            <Copy className="h-4 w-4" />
          )}
        </Button>
      </div>
      <div className="relative flex-1 overflow-hidden">
        <CodeMirror
          ref={editorRef}
          value={value}
          height="100%"
          theme={theme === 'dark' ? githubDark : githubLight}
          onChange={handleChange}
          editable={true}
          readOnly={!editable}
          extensions={[json()]}
          className={cn(
            'h-full overflow-auto [&_.cm-gutters]:border-none [&_.cm-gutters]:bg-white [&_.cm-focused]:outline-none dark:[&_.cm-gutters]:bg-[#0d1117]',
            !editable && 'cursor-text select-text'
          )}
          basicSetup={{
            lineNumbers: true,
            highlightActiveLineGutter: true,
            highlightSpecialChars: true,
            history: editable,
            foldGutter: true,
            drawSelection: true,
            dropCursor: editable,
            allowMultipleSelections: true,
            indentOnInput: editable,
            syntaxHighlighting: true,
            bracketMatching: true,
            closeBrackets: editable,
            autocompletion: editable,
            rectangularSelection: true,
            crosshairCursor: false,
            highlightActiveLine: editable,
            highlightSelectionMatches: true,
            closeBracketsKeymap: editable,
            defaultKeymap: true,
            searchKeymap: false,
            historyKeymap: editable,
            foldKeymap: true,
            completionKeymap: editable,
            lintKeymap: editable
          }}
        />
      </div>
    </div>
  );
}
