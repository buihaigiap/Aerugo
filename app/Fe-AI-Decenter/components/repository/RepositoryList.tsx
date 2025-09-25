import React from 'react';
import { Repository, Organization } from '../../types';
import RepositoryListItem from './RepositoryListItem';
import { ServerStackIcon } from '../icons/ServerStackIcon';

interface RepositoryListProps {
  repositories: Repository[];
  organization?: Organization;
  onSelectRepository: (repository: Repository) => void;
}

const RepositoryList: React.FC<RepositoryListProps> = ({ repositories, organization, onSelectRepository }) => {
  if (repositories.length === 0) {
    return (
      <div className="text-center py-16 px-4 bg-slate-800/50 border-2 border-dashed border-slate-700/80 rounded-xl">
        <ServerStackIcon className="mx-auto h-16 w-16 text-slate-500" />
        <h3 className="mt-4 text-xl font-semibold text-slate-300">No Repositories Found</h3>
        <p className="text-slate-400 mt-2">Try adjusting your search or create a new repository in this organization.</p>
      </div>
    );
  }
  return (
    <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
      {repositories.map(repo => (
        <RepositoryListItem 
            key={repo.id} 
            repository={repo} 
            organization={organization}
            onSelect={onSelectRepository}
        />
      ))}
    </div>
  );
};

export default RepositoryList;