import React, { useState } from 'react';
import { ClipboardIcon } from '../icons/ClipboardIcon';

interface CommandSnippetProps {
  command: string;
}

const CommandSnippet: React.FC<CommandSnippetProps> = ({ command }) => {
  const [copyStatus, setCopyStatus] = useState('Copy');

  const handleCopy = () => {
    navigator.clipboard.writeText(command).then(() => {
      setCopyStatus('Copied!');
      setTimeout(() => setCopyStatus('Copy'), 2000);
    }).catch(err => {
      console.error('Failed to copy text: ', err);
      setCopyStatus('Failed');
      setTimeout(() => setCopyStatus('Copy'), 2000);
    });
  };

  return (
    <div className="relative bg-slate-900/70 text-slate-300 rounded-md p-3 font-mono text-sm">
      <pre className="overflow-x-auto pr-12">
        <code>{command}</code>
      </pre>
      <button
        onClick={handleCopy}
        title="Copy command"
        className="absolute right-2 top-1/2 -translate-y-1/2 p-2 rounded-md text-slate-400 hover:bg-slate-700 hover:text-slate-100 transition-colors"
        aria-label={copyStatus}
      >
        <ClipboardIcon className="w-5 h-5" />
      </button>
    </div>
  );
};

export default CommandSnippet;