import React from 'react';
import { Organization } from '../../types';

interface OrganizationSelectorProps {
  organizations: Organization[];
  isLoading: boolean;
  error: string | null;
  selectedOrganizationId: number | null;
  onOrganizationSelect: (orgId: number | null) => void;
}

const OrganizationSelector: React.FC<OrganizationSelectorProps> = ({ 
  organizations, 
  isLoading, 
  error, 
  selectedOrganizationId,
  onOrganizationSelect 
}) => {

  const handleChange = (event: React.ChangeEvent<HTMLSelectElement>) => {
    const value = event.target.value;
    // When "All Organizations" is selected, the value is an empty string.
    // `Number('')` is 0, so we check for this case to pass `null`.
    const orgId = value ? Number(value) : null;
    onOrganizationSelect(orgId);
  };
  
  const value = selectedOrganizationId ?? "";

  if (isLoading) {
    return (
       <select
        disabled
        className="block w-full max-w-xs px-4 py-2 bg-slate-700 border border-slate-600 rounded-md text-slate-400 focus:outline-none sm:text-sm"
       >
         <option>Loading Orgs...</option>
       </select>
    );
  }

  if (error) {
    return (
      <select
        disabled
        className="block w-full max-w-xs px-4 py-2 bg-red-900/50 border border-red-700 rounded-md text-red-300 focus:outline-none sm:text-sm"
       >
         <option>{error}</option>
       </select>
    );
  }

  return (
    <select
      value={value}
      onChange={handleChange}
      className="block w-full max-w-xs px-4 py-2 bg-slate-700 border border-slate-600 rounded-md text-slate-100 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm"
      aria-label="Select an organization"
    >
      {organizations.length === 0 ? (
        <option value="">No organizations found</option>
      ) : (
        <>
          <option value="">All Organizations</option>
          {organizations.map(org => (
            <option key={org.id} value={org.id}>
              {org.name}
            </option>
          ))}
        </>
      )}
    </select>
  );
};

export default OrganizationSelector;