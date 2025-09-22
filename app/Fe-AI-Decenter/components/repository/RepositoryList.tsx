import React from 'react';
import { Repository } from '../../types';
import RepositoryListItem from './RepositoryListItem';

interface RepositoryListProps {
  repositories: Repository[];
  organizationName?: string;
  onSelectRepository: (repository: Repository) => void;
}

const RepositoryList: React.FC<RepositoryListProps> = ({ repositories, organizationName, onSelectRepository }) => {
  if (repositories.length === 0) {
    return (
      <div className="text-center py-10 px-4">
        <h3 className="text-lg font-medium text-slate-300">No Repositories Found</h3>
        <p className="text-slate-400 mt-1">Try adjusting your search or create a new repository.</p>
      </div>
    );
  }
  return (
    <div className="space-y-4">
      {repositories.map(repo => (
        <RepositoryListItem 
            key={repo.id} 
            repository={repo} 
            organizationName={organizationName}
            onSelect={onSelectRepository}
        />
      ))}
    </div>
  );
};

export default RepositoryList;