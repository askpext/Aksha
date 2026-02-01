import { useState, useEffect, useCallback, useRef } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { enable, isEnabled } from "@tauri-apps/plugin-autostart";
import type { SearchResult } from './types';
import SearchBar from './components/SearchBar';
import ResultsList from './components/ResultsList';
import ContextMenu from './components/ContextMenu';
import './App.css';

interface ContextMenuState {
  visible: boolean;
  x: number;
  y: number;
  result: SearchResult | null;
}

function App() {
  const [query, setQuery] = useState('');
  const [results, setResults] = useState<SearchResult[]>([]);
  const [selectedIndex, setSelectedIndex] = useState(0);
  const [isLoading, setIsLoading] = useState(false);
  const [contextMenu, setContextMenu] = useState<ContextMenuState>({
    visible: false,
    x: 0,
    y: 0,
    result: null,
  });


  const searchTimeoutRef = useRef<number | null>(null);

  useEffect(() => {
    const initAutoStart = async () => {
      try {
        if (!(await isEnabled())) {
          await enable();
          console.debug("Auto-start enabled");
        }
      } catch (error) {
        console.error("Failed to enable auto-start:", error);
      }
    };
    initAutoStart();
  }, []);

  // Close context menu on click outside
  useEffect(() => {
    const handleClick = () => setContextMenu({ ...contextMenu, visible: false });
    window.addEventListener('click', handleClick);
    return () => window.removeEventListener('click', handleClick);
  }, [contextMenu]);

  // Debounced search & Calculator
  useEffect(() => {
    if (searchTimeoutRef.current) {
      clearTimeout(searchTimeoutRef.current);
    }

    if (query.trim() === '') {
      setResults([]);
      setSelectedIndex(0);
      return;
    }

    // Calculator Logic
    const mathRegex = /^[\d\s\+\-\*\/\(\)\.]*$/;
    const isMath = mathRegex.test(query) && /\d/.test(query) && /[\+\-\*\/]/.test(query); // Must have numbers and operators

    setIsLoading(true);
    searchTimeoutRef.current = window.setTimeout(async () => {
      let calcResult: SearchResult | null = null;

      if (isMath) {
        try {
          // eslint-disable-next-line no-new-func
          const result = new Function(`return ${query}`)();
          if (isFinite(result)) {
            calcResult = {
              path: result.toString(),
              name: `= ${result}`,
              extension: 'calc',
              size: 0,
              modified: Date.now() / 1000,
              score: 9999,
            };
          }
        } catch (e) {
          // Ignore invalid math
          console.debug('Invalid math expression', e);
        }
      }

      try {
        const searchResults = await invoke<SearchResult[]>('search_files', { query });

        if (calcResult) {
          setResults([calcResult, ...searchResults]);
        } else {
          setResults(searchResults);
        }

        setSelectedIndex(0);
      } catch (error) {
        console.error('Search error:', error);
        setResults(calcResult ? [calcResult] : []);
      } finally {
        setIsLoading(false);
      }
    }, 150);

    return () => {
      if (searchTimeoutRef.current) {
        clearTimeout(searchTimeoutRef.current);
      }
    };
  }, [query]);

  // Handle keyboard navigation
  const handleKeyDown = useCallback(
    async (e: React.KeyboardEvent) => {
      if (e.key === 'Escape') {
        const window = getCurrentWindow();
        await window.hide();
        setQuery('');
        setResults([]);
        setSelectedIndex(0);
      } else if (e.key === 'ArrowDown') {
        e.preventDefault();
        setSelectedIndex((prev) => Math.min(prev + 1, results.length - 1));
      } else if (e.key === 'ArrowUp') {
        e.preventDefault();
        setSelectedIndex((prev) => Math.max(prev - 1, 0));
      } else if (e.key === 'Enter' && results.length > 0) {
        e.preventDefault();
        const selectedFile = results[selectedIndex];

        if (selectedFile.extension === 'calc') {
          await navigator.clipboard.writeText(selectedFile.path);
          setQuery(''); // Clear query to show "copied" feedback ideally, or just reset
          // For now just reset
          const window = getCurrentWindow();
          await window.hide();
        } else {
          try {
            await invoke('open_file', { path: selectedFile.path });
            const window = getCurrentWindow();
            await window.hide();
            setQuery('');
            setResults([]);
            setSelectedIndex(0);
          } catch (error) {
            console.error('Error opening file:', error);
          }
        }
      }
    },
    [results, selectedIndex]
  );

  const handleContextMenu = (e: React.MouseEvent, result: SearchResult) => {
    e.preventDefault();
    setContextMenu({
      visible: true,
      x: e.clientX,
      y: e.clientY,
      result,
    });
  };

  const handleOpen = async () => {
    if (contextMenu.result) {
      if (contextMenu.result.extension === 'calc') {
        await navigator.clipboard.writeText(contextMenu.result.path);
      } else {
        await invoke('open_file', { path: contextMenu.result.path });
      }
      const window = getCurrentWindow();
      await window.hide();
    }
  };

  const handleShowInFolder = async () => {
    if (contextMenu.result && contextMenu.result.extension !== 'calc') {
      await invoke('show_in_folder', { path: contextMenu.result.path });
      /* 
         We DON'T hide the window here, as the user might want to come back.
         But usually opening explorer takes focus, so it might hide if on-blur is enabled.
         Current on-blur is disabled in main.rs, so it stays open.
      */
      setContextMenu({ ...contextMenu, visible: false });
    }
  };

  const handleCopyPath = async () => {
    if (contextMenu.result) {
      await navigator.clipboard.writeText(contextMenu.result.path);
      setContextMenu({ ...contextMenu, visible: false });
    }
  };

  return (
    <div className="app-container" onKeyDown={handleKeyDown}>
      <div className="search-wrapper">
        <SearchBar
          query={query}
          onChange={setQuery}
          isLoading={isLoading}
        />
        {results.length > 0 && (
          <ResultsList
            results={results}
            selectedIndex={selectedIndex}
            onSelect={setSelectedIndex}
            onContextMenu={handleContextMenu}
          />
        )}
      </div>
      {contextMenu.visible && (
        <ContextMenu
          x={contextMenu.x}
          y={contextMenu.y}
          onClose={() => setContextMenu({ ...contextMenu, visible: false })}
          onOpen={handleOpen}
          onShowInFolder={handleShowInFolder}
          onCopyPath={handleCopyPath}
        />
      )}
    </div>
  );
}

export default App;
