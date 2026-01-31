import type { SearchResult } from '../types';
import { invoke } from '@tauri-apps/api/core';
import { getCurrentWindow } from '@tauri-apps/api/window';
import FileIcon from './FileIcon';
import './ResultsList.css';

interface ResultsListProps {
    results: SearchResult[];
    selectedIndex: number;
    onSelect: (index: number) => void;
    onContextMenu: (e: React.MouseEvent, result: SearchResult) => void;
}

function ResultsList({ results, selectedIndex, onSelect, onContextMenu }: ResultsListProps) {
    const formatFileSize = (bytes: number): string => {
        if (bytes < 1024) return `${bytes} B`;
        if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
        if (bytes < 1024 * 1024 * 1024) return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
        return `${(bytes / (1024 * 1024 * 1024)).toFixed(1)} GB`;
    };

    const formatDate = (timestamp: number): string => {
        const date = new Date(timestamp * 1000);
        const now = new Date();
        const diffDays = Math.floor((now.getTime() - date.getTime()) / (1000 * 60 * 60 * 24));

        if (diffDays === 0) return 'Today';
        if (diffDays === 1) return 'Yesterday';
        if (diffDays < 7) return `${diffDays} days ago`;
        if (diffDays < 30) return `${Math.floor(diffDays / 7)} weeks ago`;
        if (diffDays < 365) return `${Math.floor(diffDays / 30)} months ago`;
        return date.toLocaleDateString();
    };

    const getFileName = (path: string): string => {
        return path.split('\\').pop() || path;
    };

    const getFilePath = (path: string): string => {
        const parts = path.split('\\');
        parts.pop();
        return parts.join('\\');
    };

    const handleFileClick = async (result: SearchResult) => {
        try {
            if (result.extension === 'calc') {
                await navigator.clipboard.writeText(result.path);
            } else {
                await invoke('open_file', { path: result.path });
            }
            const window = getCurrentWindow();
            await window.hide();
        } catch (error) {
            console.error('Error opening file:', error);
        }
    };

    return (
        <div className="results-list hm-results-container slide-up">
            <div className="results-content">
                {results.map((result, index) => (
                    <div
                        key={result.path}
                        className={`result-item ${index === selectedIndex ? 'selected' : ''}`}
                        onMouseEnter={() => onSelect(index)}
                        onClick={() => handleFileClick(result)}
                        onContextMenu={(e) => onContextMenu(e, result)}
                    >
                        <FileIcon extension={result.extension} />
                        <div className="result-info">
                            <div className="result-name">{getFileName(result.path)}</div>
                            <div className="result-meta">
                                <span className="result-path">{getFilePath(result.path)}</span>
                                <span className="result-separator">•</span>
                                <span className="result-size">{formatFileSize(result.size)}</span>
                                <span className="result-separator">•</span>
                                <span className="result-date">{formatDate(result.modified)}</span>
                            </div>
                        </div>
                    </div>
                ))}
            </div>
        </div>
    );
}

export default ResultsList;
