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
    <div className="relative not-prose my-4 bg-slate-800/70 border border-slate-700 rounded-lg">
      <pre className={`language-${language} overflow-x-auto p-4 text-sm`}>
        <code>{code}</code>
      </pre>
      <button
        onClick={handleCopy}
        title="Copy command"
        className="absolute right-2 top-2 p-2 rounded-md text-slate-400 hover:bg-slate-700 hover:text-slate-100 transition-colors"
        aria-label={copyStatus}
      >
        <ClipboardIcon className="w-5 h-5" />
      </button>
    </div>
  );
};

export default CodeBlock;
