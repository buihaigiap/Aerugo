import React, { useState, useEffect, useMemo } from 'react';
import { fetchRepositories, fetchRepositoriesByNamespace } from '../../services/api';
import { Repository } from '../../types';
import RepositoryList from './RepositoryList';
import CreateRepositoryForm from './CreateRepositoryForm';
import RepositoryDetail from './RepositoryDetail';
import { SearchIcon } from '../icons/SearchIcon';
import Button from '../Button';
import { PlusIcon } from '../icons/PlusIcon';

interface RepositoryBrowserProps {
  token: string;
  organizationName?: string;
}

const RepositoryBrowser: React.FC<RepositoryBrowserProps> = ({ token, organizationName }) => {
  // State for the repositories shown in the "My Repositories" section.
  // This is context-dependent (all repos vs. repos in a specific org).
  const [contextRepositories, setContextRepositories] = useState<Repository[]>([]);
  
  // State for ALL public repositories, used for the "Community Repositories" section.
  // This is only populated when viewing "All Organizations".
  const [allPublicRepositories, setAllPublicRepositories] = useState<Repository[]>([]);

  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [searchTerm, setSearchTerm] = useState('');
  const [showCreateForm, setShowCreateForm] = useState(false);
  const [viewingRepository, setViewingRepository] = useState<Repository | null>(null);

  useEffect(() => {
    // Reset views when the organization context changes
    setSearchTerm('');
    setShowCreateForm(false);
    setViewingRepository(null);

    const getRepositories = async () => {
      setIsLoading(true);
      setError(null);
      try {
        if (organizationName) {
          // A specific organization is selected. Fetch ONLY its repositories.
          const orgRepos = await fetchRepositoriesByNamespace(organizationName, token);
          setContextRepositories(Array.isArray(orgRepos) ? orgRepos : []);
          // Community repos are not shown in this view, so clear the list.
          setAllPublicRepositories([]); 
        } else {
          // "All Organizations" is selected. Fetch everything to show both "My Repos" and "Community Repos".
          const allRepos = await fetchRepositories(token);
          const reposArray = Array.isArray(allRepos) ? allRepos : [];
          
          setContextRepositories(reposArray);
          setAllPublicRepositories(reposArray.filter(r => r.is_public));
        }
      } catch (err) {
        setError('Failed to load repositories.');
        setContextRepositories([]);
        setAllPublicRepositories([]);
        console.error(err);
      } finally {
        setIsLoading(false);
      }
    };

    getRepositories();
  }, [token, organizationName]);

  const handleCreationSuccess = (newRepo: Repository) => {
    setShowCreateForm(false);
    setViewingRepository(newRepo);
    // Manually add the new repo to the state to avoid a full refetch
    setContextRepositories(prev => [newRepo, ...prev.filter(r => r.id !== newRepo.id)]);
    // Only update community list if it's visible (i.e., in "All Orgs" view)
    if (newRepo.is_public && !organizationName) {
        setAllPublicRepositories(prev => [newRepo, ...prev.filter(r => r.id !== newRepo.id)]);
    }
  };

  const { myRepositories, communityRepositories } = useMemo(() => {
    const myFiltered = contextRepositories.filter(repo =>
      repo.name.toLowerCase().includes(searchTerm.toLowerCase())
    );

    const communityFiltered = allPublicRepositories.filter(repo =>
      repo.name.toLowerCase().includes(searchTerm.toLowerCase())
    );

    return { myRepositories: myFiltered, communityRepositories: communityFiltered };
  }, [contextRepositories, allPublicRepositories, searchTerm]);
  
  const handleBackToList = () => {
    setViewingRepository(null);
  };
  
  const renderContent = () => {
    if (isLoading) {
      return <div className="text-center py-8 text-slate-400">Loading repositories...</div>;
    }
    if (error) {
      return <div className="text-center py-8 text-red-500">{error}</div>;
    }

    const orgNameForDetail = organizationName || viewingRepository?.organization?.name;

    if (viewingRepository && orgNameForDetail) {
      return (
        <RepositoryDetail
          token={token}
          repository={viewingRepository}
          organizationName={orgNameForDetail}
          onBack={handleBackToList}
        />
      );
    }
    if (showCreateForm && organizationName) { // Can only create if we know the org
      return (
        <CreateRepositoryForm 
            token={token} 
            organizationName={organizationName}
            onSuccess={handleCreationSuccess}
            onCancel={() => setShowCreateForm(false)}
        />
      );
    }
    return (
      <div className="space-y-8">
        <div>
          <h3 className="text-xl font-semibold text-slate-200 border-b border-slate-700 pb-2 mb-4">
            My Repositories
            {organizationName && <span className="text-base font-normal text-slate-400"> in {organizationName}</span>}
          </h3>
          <RepositoryList 
            repositories={myRepositories} 
            organizationName={organizationName} 
            onSelectRepository={(repo) => setViewingRepository(repo)}
          />
        </div>

        {/* Only show Community Repositories if no specific organization is selected */}
        {!organizationName && (
          <div>
            <h3 className="text-xl font-semibold text-slate-200 border-b border-slate-700 pb-2 mb-4">
              Community Repositories
            </h3>
            <RepositoryList 
              repositories={communityRepositories} 
              organizationName={undefined} 
              onSelectRepository={(repo) => setViewingRepository(repo)}
            />
          </div>
        )}
      </div>
    );
  };

  return (
    <div className="bg-slate-800 border border-slate-700 rounded-lg p-6 shadow-lg">
      {!viewingRepository && !showCreateForm && (
        <header className="flex flex-col md:flex-row justify-between items-center mb-6 gap-4">
          <h2 className="text-2xl font-bold text-slate-50">Repositories</h2>
          <div className="flex items-center gap-4 w-full md:w-auto">
            <div className="relative flex-grow">
              <div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                 <SearchIcon className="h-5 w-5 text-slate-400" />
              </div>
              <input
                type="text"
                placeholder="Search repositories..."
                value={searchTerm}
                onChange={(e) => setSearchTerm(e.target.value)}
                className="block w-full pl-10 pr-4 py-2 bg-slate-700 border border-slate-600 rounded-md text-slate-100 placeholder-slate-400 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 sm:text-sm"
              />
            </div>
            <Button 
                onClick={() => setShowCreateForm(true)} 
                fullWidth={false} 
                className="whitespace-nowrap flex-shrink-0"
                disabled={!organizationName}
                title={!organizationName ? "Select an organization to create a repository" : ""}
            >
                <PlusIcon className="w-5 h-5 -ml-1 mr-2" />
                Create Repository
            </Button>
          </div>
        </header>
      )}
      <main>
        {renderContent()}
      </main>
    </div>
  );
};

export default RepositoryBrowser;