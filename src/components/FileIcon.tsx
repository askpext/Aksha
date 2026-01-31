import './FileIcon.css';

interface FileIconProps {
    extension: string;
}

function FileIcon({ extension }: FileIconProps) {
    const getIconContent = (ext: string) => {
        const imageExts = ['jpg', 'jpeg', 'png', 'gif', 'bmp', 'svg', 'webp'];
        const videoExts = ['mp4', 'avi', 'mov', 'mkv', 'webm'];
        const audioExts = ['mp3', 'wav', 'flac', 'aac', 'm4a'];
        const codeExts = ['js', 'ts', 'jsx', 'tsx', 'py', 'java', 'cpp', 'c', 'rs', 'go', 'html', 'css', 'json', 'xml', 'yaml', 'yml'];
        const docExts = ['doc', 'docx', 'pdf', 'txt', 'md'];
        const appExts = ['exe', 'lnk', 'msi', 'bat', 'cmd'];
        const archiveExts = ['zip', 'rar', '7z', 'tar', 'gz'];

        // Folder: Simple Folder Icon
        if (ext === 'folder') {
            return (
                <>
                    <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z" />
                </>
            );
        }

        /* 
         * Minimalist Monochrome SVGs 
         * All icons designed on 24x24 grid
         */

        // Image: Simple Picture Frame
        if (imageExts.includes(ext)) {
            return (
                <>
                    <rect x="3" y="3" width="18" height="18" rx="2" ry="2" />
                    <circle cx="8.5" cy="8.5" r="1.5" />
                    <polyline points="21 15 16 10 5 21" />
                </>
            );
        }

        // Video: Film Strip / Play Button
        if (videoExts.includes(ext)) {
            return (
                <>
                    <rect x="2" y="2" width="20" height="20" rx="2.18" ry="2.18" />
                    <line x1="7" y1="2" x2="7" y2="22" />
                    <line x1="17" y1="2" x2="17" y2="22" />
                    <line x1="2" y1="12" x2="22" y2="12" />
                    <line x1="2" y1="7" x2="7" y2="7" />
                    <line x1="2" y1="17" x2="7" y2="17" />
                    <line x1="17" y1="17" x2="22" y2="17" />
                    <line x1="17" y1="7" x2="22" y2="7" />
                </>
            );
        }

        // Audio: Musical Note
        if (audioExts.includes(ext)) {
            return (
                <>
                    <path d="M9 18V5l12-2v13" />
                    <circle cx="6" cy="18" r="3" />
                    <circle cx="18" cy="16" r="3" />
                </>
            );
        }

        // Code: Terminal / Code Brackets
        if (codeExts.includes(ext)) {
            return (
                <>
                    <polyline points="16 18 22 12 16 6" />
                    <polyline points="8 6 2 12 8 18" />
                </>
            );
        }

        // App/Executable: Window/Screen or Lightning
        if (appExts.includes(ext)) {
            return (
                <>
                    <rect x="2" y="3" width="20" height="14" rx="2" ry="2" />
                    <line x1="8" y1="21" x2="16" y2="21" />
                    <line x1="12" y1="17" x2="12" y2="21" />
                </>
            );
        }

        // Archive: Box / Zipper
        if (archiveExts.includes(ext)) {
            return (
                <>
                    <polyline points="21 8 21 21 3 21 3 8" />
                    <rect x="1" y="3" width="22" height="5" />
                    <line x1="10" y1="12" x2="14" y2="12" />
                </>
            );
        }

        // Default / Document: File Page
        if (docExts.includes(ext) || true) { // Fallback for all other files
            return (
                <>
                    <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" />
                    <polyline points="14 2 14 8 20 8" />
                </>
            );
        }
    };

    return (
        <div className="file-icon">
            <svg
                width="20"
                height="20"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                strokeWidth="1.5"
                strokeLinecap="round"
                strokeLinejoin="round"
            >
                {getIconContent(extension.toLowerCase())}
            </svg>
            {/* H&M Style: No text overlay, cleaner look. Tooltip or separate text handles extension. */}
        </div>
    );
}

export default FileIcon;
