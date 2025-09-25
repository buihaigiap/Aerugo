import React, { useState, useEffect, useCallback } from 'react';
import { Organization, OrganizationMember, User } from '../../types';
import { fetchOrganizationMembers, fetchOrganizationDetails } from '../../services/api';
import MemberList from './MemberList';
import Button from '../Button';
import { PlusIcon } from '../icons/PlusIcon';
import AddMemberForm from './AddMemberForm';
import OrganizationSettings from './OrganizationSettings';
import { UsersIcon } from '../icons/UsersIcon';
import { CogIcon } from '../icons/CogIcon';

interface OrganizationDetailProps {
  token: string;
  currentUser: User;
  organization: Organization;
  onDataChange: () => void;
}

type Tab = 'members' | 'settings';

const OrganizationDetail: React.FC<OrganizationDetailProps> = ({ token, currentUser, organization, onDataChange }) => {
  const [activeTab, setActiveTab] = useState<Tab>('members');
  const [detailedOrg, setDetailedOrg] = useState<Organization>(organization);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const [members, setMembers] = useState<OrganizationMember[]>([]);
  const [isLoadingMembers, setIsLoadingMembers] = useState(true);
  const [errorMembers, setErrorMembers] = useState<string | null>(null);

  const getDetails = useCallback(async () => {
    setIsLoading(true);
    setError(null);
    try {
        const data = await fetchOrganizationDetails(organization.id, token);
        setDetailedOrg(data);
    } catch (err) {
        setError('Failed to load up-to-date organization details.');
        console.error(err);
    } finally {
        setIsLoading(false);
    }
  }, [organization.id, token]);

  const getMembers = useCallback(async () => {
    setIsLoadingMembers(true);
    setErrorMembers(null);
    try {
      const memberList = await fetchOrganizationMembers(organization.id, token);
      setMembers(memberList);
    } catch (err) {
      setErrorMembers('Failed to load members.');
      console.error(err);
    } finally {
      setIsLoadingMembers(false);
    }
  }, [organization.id, token]);


  useEffect(() => {
    getDetails();
    getMembers();
  }, [getDetails, getMembers]);

  const handleSettingsChange = () => {
      getDetails();
      onDataChange();
  }
  
  const handleMembersChanged = () => {
    getMembers();
  }

  const currentUserRole = members.find(m => m.user_id === currentUser.id)?.role.toLowerCase();

  return (
    <div className="bg-slate-800/70 border border-slate-700/80 rounded-xl shadow-2xl shadow-slate-950/50">
      <header className="p-6 border-b border-slate-700/80 flex items-center space-x-5">
        <div className="flex-shrink-0">
            <span className="h-16 w-16 rounded-full bg-slate-700 flex items-center justify-center border border-slate-600">
                <img src="/components/icons/logo.png" alt="Organization Logo" className="h-[100px] w-[100px]" />
            </span>
        </div>
        <div className="flex-1">
            <h2 className="text-2xl font-bold text-slate-50">{detailedOrg.display_name}</h2>
            <p className="text-slate-400">@{detailedOrg.name}</p>
        </div>
      </header>
      
      <div className="border-b border-slate-700/80">
          <nav className="flex space-x-4 px-6" aria-label="Tabs">
            <TabButton 
                icon={<UsersIcon className="w-5 h-5 mr-2" />} 
                label="Members" 
                isActive={activeTab === 'members'} 
                onClick={() => setActiveTab('members')} 
            />
            <TabButton 
                icon={<CogIcon className="w-5 h-5 mr-2" />} 
                label="Settings" 
                isActive={activeTab === 'settings'} 
                onClick={() => setActiveTab('settings')} 
            />
          </nav>
      </div>


      <div className="p-6">
        {isLoading ? (
            <div className="text-center py-8 text-slate-400">Loading details...</div>
        ) : error ? (
            <div className="text-center py-8 text-red-500">{error}</div>
        ) : (
          <div key={activeTab} className="animate-fade-in-up">
            {activeTab === 'members' && (
                <MembersView 
                    token={token} 
                    organization={detailedOrg} 
                    currentUser={currentUser}
                    members={members}
                    isLoading={isLoadingMembers}
                    error={errorMembers}
                    currentUserRole={currentUserRole}
                    onDataChange={handleMembersChanged}
                />
            )}
            {activeTab === 'settings' && (
                <OrganizationSettings 
                    token={token} 
                    organization={detailedOrg}
                    currentUserRole={currentUserRole}
                    onOrganizationUpdated={handleSettingsChange}
                    onOrganizationDeleted={onDataChange}
                />
            )}
          </div>
        )}
      </div>
    </div>
  );
};

const TabButton: React.FC<{icon: React.ReactNode, label: string, isActive: boolean, onClick: () => void}> = ({ icon, label, isActive, onClick }) => (
    <button
      onClick={onClick}
      className={`flex items-center whitespace-nowrap py-3 px-1 border-b-2 font-medium text-sm transition-colors duration-200 focus:outline-none focus-visible:ring-2 focus-visible:ring-indigo-500 focus-visible:ring-offset-2 focus-visible:ring-offset-slate-800 rounded-sm
        ${
          isActive
            ? 'border-indigo-500 text-indigo-400'
            : 'border-transparent text-slate-400 hover:text-slate-200 hover:border-slate-500'
        }`}
    >
        {icon}
        <span>{label}</span>
    </button>
);

const MembersView: React.FC<{
    token: string, 
    organization: Organization, 
    currentUser: User,
    members: OrganizationMember[],
    isLoading: boolean,
    error: string | null,
    currentUserRole?: string,
    onDataChange: () => void,
}> = ({ token, organization, currentUser, members, isLoading, error, currentUserRole, onDataChange }) => {
    const [showAddForm, setShowAddForm] = useState(false);

    const handleSuccess = () => {
        setShowAddForm(false);
        onDataChange();
    }

    return (
        <div className="space-y-6">
            <div className="flex items-center justify-between">
                <h3 className="text-xl font-bold text-slate-50">Members</h3>
                {!showAddForm && (currentUserRole === 'owner' || currentUserRole === 'admin') && (
                    <Button onClick={() => setShowAddForm(true)} fullWidth={false}>
                        <PlusIcon className="w-5 h-5 -ml-1 mr-2" />
                        Add Member
                    </Button>
                )}
            </div>
            {showAddForm && (
                <AddMemberForm 
                    token={token}
                    orgId={organization.id}
                    onSuccess={handleSuccess}
                    onCancel={() => setShowAddForm(false)}
                />
            )}
             {isLoading ? (
                <div className="text-center py-8 text-slate-400">Loading members...</div>
                ) : error ? (
                <div className="text-center py-8 text-red-500">{error}</div>
                ) : (
                <MemberList 
                    members={members}
                    currentUser={currentUser}
                    currentUserRole={currentUserRole}
                    orgId={organization.id}
                    token={token}
                    onDataChange={onDataChange}
                 />
            )}
        </div>
    );
};

export default OrganizationDetail;