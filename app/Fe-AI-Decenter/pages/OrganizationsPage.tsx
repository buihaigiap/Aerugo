import React, { useState, useEffect, useCallback } from 'react';
import OrganizationsManager from '../components/organization/OrganizationsManager';
import { Organization, User } from '../types';
import { fetchOrganizations } from '../services/api';

interface OrganizationsPageProps {
  token: string;
  currentUser: User;
}

const OrganizationsPage: React.FC<OrganizationsPageProps> = ({ token, currentUser }) => {
  const [organizations, setOrganizations] = useState<Organization[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const getOrganizations = useCallback(async () => {
    try {
      setIsLoading(true);
      setError(null);
      const orgs = await fetchOrganizations(token);
      setOrganizations(Array.isArray(orgs) ? orgs : []);
    } catch (err) {
      setError('Failed to load organizations.');
      setOrganizations([]);
      console.error(err);
    } finally {
      setIsLoading(false);
    }
  }, [token]);

  useEffect(() => {
    getOrganizations();
  }, [getOrganizations]);

  return (
    <OrganizationsManager 
        token={token}
        currentUser={currentUser}
        organizations={organizations}
        isLoading={isLoading}
        error={error}
        onDataChange={getOrganizations}
    />
  );
};

export default OrganizationsPage;
