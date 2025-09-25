import React, { useState } from 'react';
import { Repository, Organization } from '../../types';
import { LockIcon } from '../icons/LockIcon';
import { GlobeIcon } from '../icons/GlobeIcon';
import { ClipboardIcon } from '../icons/ClipboardIcon';
import { ServerStackIcon } from '../icons/ServerStackIcon';

interface RepositoryListItemProps {
  repository: Repository;
  organization?: Organization;
  onSelect: (repository: Repository) => void;
}

const RepositoryListItem: React.FC<RepositoryListItemProps> = ({ repository, organization, onSelect }) => {
  const isPrivate = !repository.is_public;
  const [copyStatus, setCopyStatus] = useState('Copy');

  const orgName = repository.organization?.name || organization?.name;
  const pullCommand = `docker pull aerugo.io/${orgName}/${repository.name}:latest`;

  const handleCopy = (e: React.MouseEvent) => {
    e.stopPropagation(); // Prevent card click when copying
    navigator.clipboard.writeText(pullCommand).then(() => {
      setCopyStatus('Copied!');
      setTimeout(() => setCopyStatus('Copy'), 2000);
    }).catch(err => {
      console.error('Failed to copy text: ', err);
      setCopyStatus('Failed');
       setTimeout(() => setCopyStatus('Copy'), 2000);
    });
  };

  return (
    <div 
        className="group relative bg-slate-800/50 border border-slate-700/80 rounded-lg transition-all duration-300 ease-in-out hover:shadow-2xl hover:shadow-slate-950/50 hover:border-indigo-500/50 hover:-translate-y-1 flex flex-col"
        onClick={() => onSelect(repository)}
    >
      <div className="p-5 flex-grow">
        <header className="flex items-start justify-between gap-4">
            <div className="flex items-center gap-4 flex-1 min-w-0">
                <div className="flex-shrink-0 p-2 bg-slate-700/50 rounded-md border border-slate-600">
                    <ServerStackIcon className="w-5 h-5 text-indigo-400" />
                </div>
                <button className="text-lg font-semibold text-slate-50 hover:text-indigo-400 transition-colors text-left truncate">
                  <span className="text-slate-400">{orgName} /</span> {repository.name}
                </button>
            </div>
            <span 
              className={`flex-shrink-0 inline-flex items-center px-2.5 py-1 rounded-full text-xs font-medium border ${
                isPrivate 
                ? 'bg-yellow-900/30 text-yellow-300 border-yellow-700/50' 
                : 'bg-green-900/30 text-green-300 border-green-700/50'
              }`}
            >
              {isPrivate ? <LockIcon className="w-3 h-3 mr-1.5"/> : <GlobeIcon className="w-3 h-3 mr-1.5"/> }
              {isPrivate ? 'Private' : 'Public'}
            </span>
        </header>

        <p className="mt-4 text-sm text-slate-400 h-10">
          {repository.description || 'No description provided.'}
        </p>
      </div>

      <footer className="border-t border-slate-700/80 bg-slate-900/30 px-5 py-3 rounded-b-lg">
         <div className="relative flex items-center w-full">
            <span className="text-indigo-300 font-mono text-sm truncate pr-16">{pullCommand}</span>
            <button
                onClick={handleCopy}
                title="Copy pull command"
                className="absolute right-0 top-1/2 -translate-y-1/2 flex items-center gap-2 px-2 py-1 rounded-md text-slate-400 bg-slate-700/50 hover:bg-slate-700 hover:text-slate-100 transition-colors text-xs focus:outline-none"
            >
                <ClipboardIcon className="w-4 h-4" />
                <span>{copyStatus}</span>
            </button>
        </div>
      </footer>
    </div>
  );
};

export default RepositoryListItem;