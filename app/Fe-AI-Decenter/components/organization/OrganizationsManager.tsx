
import React, { useState, useEffect } from 'react';
import { Organization, User } from '../../types';
import OrganizationList from './OrganizationList';
import OrganizationDetail from './OrganizationDetail';
import CreateOrganizationForm from './CreateOrganizationForm';
import Button from '../Button';
import { PlusIcon } from '../icons/PlusIcon';
import { BriefcaseIcon } from '../icons/BriefcaseIcon';

interface OrganizationsManagerProps {
  token: string;
  currentUser: User;
  organizations: Organization[];
  isLoading: boolean;
  error: string | null;
  onDataChange: () => void;
}

const OrganizationsManager: React.FC<OrganizationsManagerProps> = ({
  token,
  currentUser,
  organizations,
  isLoading,
  error,
  onDataChange,
}) => {
  const [selectedOrganization, setSelectedOrganization] = useState<Organization | null>(null);
  const [showCreateForm, setShowCreateForm] = useState(false);

  useEffect(() => {
    if (organizations.length > 0 && !showCreateForm) {
      if (selectedOrganization) {
        if (!organizations.find(o => o.id === selectedOrganization.id)) {
          setSelectedOrganization(organizations[0]);
        }
      } else {
        setSelectedOrganization(organizations[0]);
      }
    } else if (organizations.length === 0) {
      setSelectedOrganization(null);
    }
  }, [organizations, selectedOrganization, showCreateForm]);
  
  const handleSelectOrganization = (org: Organization) => {
    setSelectedOrganization(org);
    setShowCreateForm(false);
  };

  const handleShowCreateForm = () => {
    setSelectedOrganization(null);
    setShowCreateForm(true);
  };
  
  const handleCreationSuccess = () => {
    setShowCreateForm(false);
    onDataChange(); 
  };
  
  const handleCancelCreate = () => {
    setShowCreateForm(false);
    if (organizations.length > 0) {
        setSelectedOrganization(organizations[0]);
    }
  };

  const renderContent = () => {
    if (showCreateForm) {
      return (
        <CreateOrganizationForm
          token={token}
          onSuccess={handleCreationSuccess}
          onCancel={handleCancelCreate}
        />
      );
    }
    if (selectedOrganization) {
      return (
        <OrganizationDetail
          key={selectedOrganization.id}
          token={token}
          currentUser={currentUser}
          organization={selectedOrganization}
          onDataChange={onDataChange}
        />
      );
    }
    return (
      <div className="relative h-full min-h-[60vh] flex flex-col items-center justify-center text-center bg-slate-800/50 border border-slate-700/80 rounded-xl p-8 overflow-hidden">
        <div className="absolute inset-0 z-0 opacity-[0.03] animate-bg-grid-flow" style={{ backgroundSize: '2.5rem 2.5rem' }}></div>
        <div className="absolute inset-0 z-0 bg-indigo-950/20 [mask-image:radial-gradient(ellipse_at_center,black_20%,transparent_70%)] animate-pulse-glow"></div>
        <div className="relative z-10">
          <BriefcaseIcon className="h-20 w-20 text-slate-600 mb-4 mx-auto" />
          <h3 className="text-xl font-bold text-slate-300">Select an Organization</h3>
          <p className="text-slate-400 mt-2 max-w-sm">
            Select an organization from the list to view its details, or create a new one to get started.
          </p>
        </div>
      </div>
    );
  };

  return (
    <div className="grid grid-cols-1 lg:grid-cols-12 gap-8 items-start">
      <aside className="lg:col-span-4 xl:col-span-3 lg:sticky lg:top-24">
         <div className="bg-slate-800/50 border border-slate-700/80 rounded-xl">
            <header className="p-4 border-b border-slate-700/80 flex items-center justify-between">
                <h3 className="font-semibold text-slate-100">Your Organizations</h3>
                <Button 
                    onClick={handleShowCreateForm} 
                    fullWidth={false} 
                    className="!py-1.5 !px-2"
                    title="Create New Organization"
                >
                    <PlusIcon className="w-5 h-5" />
                </Button>
            </header>
            <div className="p-2">
                {isLoading ? (
                  <p className="text-slate-400 text-center py-4 text-sm">Loading...</p>
                ) : error ? (
                  <p className="text-red-500 text-center py-4 text-sm">{error}</p>
                ) : (
                  <OrganizationList
                    organizations={organizations}
                    selectedOrganizationId={selectedOrganization?.id || null}
                    onSelectOrganization={handleSelectOrganization}
                  />
                )}
            </div>
        </div>
      </aside>

      <main className="lg:col-span-8 xl:col-span-9">
        {renderContent()}
      </main>
    </div>
  );
};

export default OrganizationsManager;
