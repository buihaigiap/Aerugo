import React, { useState, useEffect, useCallback } from 'react';
import OrganizationSelector from '../components/organization/OrganizationSelector';
import RepositoryBrowser from '../components/repository/RepositoryBrowser';
import { Organization } from '../types';
import { fetchOrganizations } from '../services/api';

interface RepositoriesPageProps {
  token: string;
}

const RepositoriesPage: React.FC<RepositoriesPageProps> = ({ token }) => {
  const [selectedOrgId, setSelectedOrgId] = useState<number | null>(null);
  const [organizations, setOrganizations] = useState<Organization[]>([]);
  const [isLoadingOrgs, setIsLoadingOrgs] = useState(true);
  const [orgsError, setOrgsError] = useState<string | null>(null);

  const getOrganizations = useCallback(async () => {
    try {
      setIsLoadingOrgs(true);
      setOrgsError(null);
      const orgs = await fetchOrganizations(token);
      setOrganizations(Array.isArray(orgs) ? orgs : []);
    } catch (err) {
      setOrgsError('Failed to load organizations.');
      setOrganizations([]);
      console.error(err);
    } finally {
      setIsLoadingOrgs(false);
    }
  }, [token]);

  useEffect(() => {
    getOrganizations();
  }, [getOrganizations]);

  useEffect(() => {
    // This effect handles maintaining a valid selection if the list of orgs changes.
    // For example, if the currently selected organization is deleted.
    if (selectedOrgId && !organizations.some(o => o.id === selectedOrgId)) {
      // Fallback to the first organization in the list, or to "All" if no organizations are left.
      setSelectedOrgId(organizations.length > 0 ? organizations[0].id : null);
    }
  }, [organizations, selectedOrgId]);
  
  const selectedOrg = organizations.find(o => o.id === selectedOrgId);

  return (
    <div className="space-y-6">
       <div className="flex justify-between items-center px-4 sm:px-0">
          <h1 className="text-3xl font-bold text-slate-50">Repositories</h1>
          <OrganizationSelector 
              organizations={organizations}
              isLoading={isLoadingOrgs}
              error={orgsError}
              onOrganizationSelect={setSelectedOrgId} 
              selectedOrganizationId={selectedOrgId}
          />
       </div>
       <RepositoryBrowser key={selectedOrgId ?? 'all'} token={token} organizationName={selectedOrg?.name} />
    </div>
  );
};

export default RepositoriesPage;