import React from 'react';
import { Organization } from '../../types';
import OrganizationListItem from './OrganizationListItem';

interface OrganizationListProps {
  organizations: Organization[];
  selectedOrganizationId: number | null;
  onSelectOrganization: (organization: Organization) => void;
}

const OrganizationList: React.FC<OrganizationListProps> = ({ organizations, selectedOrganizationId, onSelectOrganization }) => {
  if (organizations.length === 0) {
    return (
      <div className="text-center py-10 px-4 bg-slate-800 border-2 border-dashed border-slate-700 rounded-lg">
        <h3 className="text-lg font-medium text-slate-300">No Organizations Found</h3>
        <p className="text-slate-400 mt-1">Get started by creating a new organization.</p>
      </div>
    );
  }

  return (
    <div className="bg-slate-800 border border-slate-700 rounded-lg shadow-lg">
        <ul className="divide-y divide-slate-700">
            {organizations.map(org => (
                <OrganizationListItem 
                    key={org.id} 
                    organization={org} 
                    isSelected={org.id === selectedOrganizationId}
                    onSelect={() => onSelectOrganization(org)}
                />
            ))}
        </ul>
    </div>
  );
};

export default OrganizationList;