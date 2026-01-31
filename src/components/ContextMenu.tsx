import { useEffect, useRef } from 'react';
import './ContextMenu.css';

export interface ContextMenuProps {
    x: number;
    y: number;
    onClose: () => void;
    onOpen: () => void;
    onShowInFolder: () => void;
    onCopyPath: () => void;
}

function ContextMenu({ x, y, onClose, onOpen, onShowInFolder, onCopyPath }: ContextMenuProps) {
    const menuRef = useRef<HTMLDivElement>(null);

    useEffect(() => {
        const handleClickOutside = (event: MouseEvent) => {
            if (menuRef.current && !menuRef.current.contains(event.target as Node)) {
                onClose();
            }
        };

        const handleKeyDown = (event: KeyboardEvent) => {
            if (event.key === 'Escape') {
                onClose();
            }
        };

        document.addEventListener('mousedown', handleClickOutside);
        document.addEventListener('keydown', handleKeyDown);

        return () => {
            document.removeEventListener('mousedown', handleClickOutside);
            document.removeEventListener('keydown', handleKeyDown);
        };
    }, [onClose]);

    // Adjust position to keep menu within viewport
    const style = {
        top: Math.min(y, window.innerHeight - 150),
        left: Math.min(x, window.innerWidth - 200),
    };

    return (
        <div className="context-menu" style={style} ref={menuRef}>
            <div className="context-menu-item" onClick={onOpen}>
                <span>Open</span>
            </div>
            <div className="context-menu-separator" />
            <div className="context-menu-item" onClick={onShowInFolder}>
                <span>Show in Folder</span>
            </div>
            <div className="context-menu-item" onClick={onCopyPath}>
                <span>Copy Full Path</span>
            </div>
        </div>
    );
}

export default ContextMenu;
