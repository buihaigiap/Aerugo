
import React, { useState } from 'react';
import { ClipboardIcon } from '../icons/ClipboardIcon';

interface CodeBlockProps {
  code: string;
  language?: string;
}

const CodeBlock: React.FC<CodeBlockProps> = ({ code, language }) => {
  const [copyStatus, setCopyStatus] = useState('Copy');

  const handleCopy = () => {
    navigator.clipboard.writeText(code).then(() => {
      setCopyStatus('Copied!');
      setTimeout(() => setCopyStatus('Copy'), 2000);
    }).catch(err => {
      console.error('Failed to copy text: ', err);
      setCopyStatus('Failed');
      setTimeout(() => setCopyStatus('Copy'), 2000);
    });
  };

  return (
    <div className="relative not-prose my-6 bg-slate-800/70 border border-slate-700/80 rounded-lg shadow-lg overflow-hidden">
        <div className="flex items-center justify-between px-4 py-2 bg-slate-900/50 border-b border-slate-700/80">
            <span className="text-xs text-slate-400 font-mono capitalize">{language || 'bash'}</span>
            <button
                onClick={handleCopy}
                title="Copy to clipboard"
                className="flex items-center gap-2 px-2 py-1 rounded-md text-slate-400 hover:bg-slate-700 hover:text-slate-100 transition-colors text-xs focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 focus:ring-offset-slate-900"
                aria-label={copyStatus}
            >
                <ClipboardIcon className="w-4 h-4" />
                <span className="w-12 text-left">{copyStatus}</span>
            </button>
        </div>
      <pre className={`language-${language} overflow-x-auto p-4 text-sm`}>
        <code>{code}</code>
      </pre>
    </div>
  );
};

export default CodeBlock;
