import React from 'react';
import { Organization } from '../../types';
import OrganizationListItem from './OrganizationListItem';
import { BriefcaseIcon } from '../icons/BriefcaseIcon';

interface OrganizationListProps {
  organizations: Organization[];
  selectedOrganizationId: number | null;
  onSelectOrganization: (org: Organization) => void;
}

const OrganizationList: React.FC<OrganizationListProps> = ({ organizations, selectedOrganizationId, onSelectOrganization }) => {
  if (organizations.length === 0) {
    return (
      <div className="text-center py-12 px-4">
        <BriefcaseIcon className="mx-auto h-12 w-12 text-slate-500" />
        <h3 className="mt-4 text-lg font-semibold text-slate-300">No Organizations Found</h3>
        <p className="text-slate-400 mt-1 text-sm">Get started by creating one.</p>
      </div>
    );
  }

  return (
    <ul className="space-y-0.5">
      {organizations.map(org => (
        <OrganizationListItem
          key={org.id}
          organization={org}
          isSelected={org.id === selectedOrganizationId}
          onSelect={() => onSelectOrganization(org)}
        />
      ))}
    </ul>
  );
};

export default OrganizationList;