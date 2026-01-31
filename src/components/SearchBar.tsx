import { useEffect, useRef } from 'react';
import './SearchBar.css';

interface SearchBarProps {
    query: string;
    onChange: (value: string) => void;
    isLoading: boolean;
}

function SearchBar({ query, onChange, isLoading }: SearchBarProps) {
    const inputRef = useRef<HTMLInputElement>(null);

    useEffect(() => {
        // Auto-focus on mount
        inputRef.current?.focus();
    }, []);

    return (
        <div className="search-bar hm-search-container">
            <div className="search-content">
                <div className="search-icon">
                    <svg
                        width="20"
                        height="20"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        strokeWidth="1.5"
                        strokeLinecap="square"
                        strokeLinejoin="miter"
                    >
                        <circle cx="11" cy="11" r="8" />
                        <path d="m21 21-4.35-4.35" />
                    </svg>
                </div>
                <input
                    ref={inputRef}
                    type="text"
                    className="search-input"
                    placeholder="SEARCH"
                    value={query}
                    onChange={(e) => onChange(e.target.value)}
                    spellCheck={false}
                    autoComplete="off"
                />
                {isLoading && (
                    <div className="loading-spinner">
                        <div className="spinner" />
                    </div>
                )}
            </div>
        </div>
    );
}

export default SearchBar;
