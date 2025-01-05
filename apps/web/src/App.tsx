import { useState, useEffect, useCallback } from 'react';
import { ThriftEditor } from './components/editor/ThriftEditor';
import { ParserService } from './lib/parser/service';
import { initRicoWasm } from './lib/rico';
import { EXAMPLE_THRIFT, EXAMPLE_JSON } from './lib/examples';
import { Header } from './components/layout/Header';
import {
  CONVERSION_MODES,
  type ConversionMode,
  type WasmStatus,
  type ParserStatus as ParserStatusType,
  type ParserMetrics
} from './lib/constants';

function App() {
  const [mode, setMode] = useState<ConversionMode>(
    CONVERSION_MODES.THRIFT_TO_JSON
  );
  const [input, setInput] = useState(
    mode === CONVERSION_MODES.THRIFT_TO_JSON ? EXAMPLE_THRIFT : EXAMPLE_JSON
  );
  const [output, setOutput] = useState('');
  const [wasmStatus, setWasmStatus] = useState<WasmStatus>({
    isInitializing: true,
    error: undefined
  });
  const [parserStatus, setParserStatus] = useState<ParserStatusType>({
    isLoading: false,
    error: undefined
  });
  const [parserMetrics, setParserMetrics] = useState<ParserMetrics>({});

  useEffect(() => {
    initRicoWasm()
      .then(() => {
        setWasmStatus({
          isInitializing: false,
          error: undefined
        });
      })
      .catch(error => {
        console.error('Failed to initialize Rico:', error);
        const errorMessage = 'Failed to initialize Rico WASM';
        setOutput(JSON.stringify({ error: errorMessage }, null, 2));
        setWasmStatus({
          isInitializing: false,
          error: errorMessage
        });
      });
  }, []);

  const toggleMode = () => {
    const newMode =
      mode === CONVERSION_MODES.THRIFT_TO_JSON
        ? CONVERSION_MODES.JSON_TO_THRIFT
        : CONVERSION_MODES.THRIFT_TO_JSON;
    setMode(newMode);
    setInput(
      newMode === CONVERSION_MODES.THRIFT_TO_JSON
        ? EXAMPLE_THRIFT
        : EXAMPLE_JSON
    );
    setOutput('');
  };

  const convert = useCallback(
    async (content: string) => {
      if (wasmStatus.isInitializing) return;

      setParserStatus(prev => ({ ...prev, isLoading: true, error: undefined }));
      const metrics = {
        parseTime: 0,
        stringifyTime: 0
      };

      try {
        const parseStart = performance.now();
        const parserService = ParserService.getInstance();
        const result =
          mode === CONVERSION_MODES.THRIFT_TO_JSON
            ? await parserService.parse(content)
            : await parserService.generate(content);
        metrics.parseTime = performance.now() - parseStart;

        if (!result.success) {
          setParserStatus(prev => ({
            ...prev,
            error:
              result.error ||
              `Failed to ${
                mode === CONVERSION_MODES.THRIFT_TO_JSON
                  ? 'parse Thrift IDL'
                  : 'generate Thrift from JSON'
              }`
          }));
          setOutput(JSON.stringify({ error: result.error }, null, 2));
        } else {
          metrics.stringifyTime = 0;
          const formattedOutput =
            mode === CONVERSION_MODES.THRIFT_TO_JSON
              ? JSON.stringify(JSON.parse(result.data ?? ''), null, 2)
              : result.data ?? '';
          setOutput(formattedOutput);
          setParserStatus(prev => ({
            ...prev,
            error: undefined
          }));
        }
      } catch (error: unknown) {
        const errorMessage =
          error instanceof Error ? error.message : String(error);
        setOutput(JSON.stringify({ error: errorMessage }, null, 2));
        setParserStatus(prev => ({
          ...prev,
          error: errorMessage
        }));
      } finally {
        setParserStatus(prev => ({ ...prev, isLoading: false }));
        setParserMetrics(metrics);
      }
    },
    [mode, wasmStatus.isInitializing]
  );

  useEffect(() => {
    const timer = setTimeout(() => {
      convert(input);
    }, 300);

    return () => clearTimeout(timer);
  }, [input, convert]);

  const isDisabled = wasmStatus.isInitializing;
  const isThriftToJson = mode === CONVERSION_MODES.THRIFT_TO_JSON;

  const getEditorTitle = (isInput: boolean) =>
    isInput
      ? isThriftToJson
        ? 'Thrift IDL'
        : 'JSON'
      : isThriftToJson
      ? 'JSON'
      : 'Thrift IDL';

  const getEditorSubtitle = (isInput: boolean) => {
    const type = getEditorTitle(isInput);
    return `${isInput ? 'Enter your' : 'Generated'} ${type}${
      isInput ? ' here' : ' will appear here'
    }`;
  };

  const getEditorLanguage = (isInput: boolean) =>
    isInput
      ? isThriftToJson
        ? 'thrift'
        : 'json'
      : isThriftToJson
      ? 'json'
      : 'thrift';

  return (
    <div className="min-h-screen bg-gradient-to-br from-gray-50 to-gray-100 p-6 dark:from-gray-900 dark:to-gray-800">
      <div className="mx-auto flex max-w-[1600px] flex-col gap-6">
        <Header
          isThriftToJson={isThriftToJson}
          isDisabled={isDisabled}
          onToggleMode={toggleMode}
          wasmStatus={wasmStatus}
          parserStatus={parserStatus}
          parserMetrics={parserMetrics}
        />

        <main className="grid h-[calc(100vh-12rem)] grid-cols-2 gap-6">
          <ThriftEditor
            value={input}
            onChange={setInput}
            title={getEditorTitle(true)}
            subtitle={getEditorSubtitle(true)}
            language={getEditorLanguage(true)}
          />
          <ThriftEditor
            value={output}
            editable={false}
            title={getEditorTitle(false)}
            subtitle={getEditorSubtitle(false)}
            language={getEditorLanguage(false)}
          />
        </main>
      </div>
    </div>
  );
}

export default App;
