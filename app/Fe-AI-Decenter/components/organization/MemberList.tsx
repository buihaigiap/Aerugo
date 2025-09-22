import React from 'react';
import { OrganizationMember, User } from '../../types';
import MemberListItem from './MemberListItem';
import { UsersIcon } from '../icons/UsersIcon';

interface MemberListProps {
  members: OrganizationMember[];
  currentUser: User;
  currentUserRole?: string;
  orgId: number;
  token: string;
  onDataChange: () => void;
}

const MemberList: React.FC<MemberListProps> = ({ members, currentUser, currentUserRole, orgId, token, onDataChange }) => {
  if (members.length === 0) {
    return (
      <div className="text-center py-10 px-4 border-2 border-dashed border-slate-700 rounded-lg">
        <UsersIcon className="mx-auto h-12 w-12 text-slate-500" />
        <h3 className="mt-2 text-lg font-medium text-slate-300">No Members Found</h3>
        <p className="text-slate-400 mt-1">This organization doesn't have any members yet.</p>
      </div>
    );
  }

  // Define the hierarchical order of roles with lowercase keys for robust sorting
  const roleOrder: { [key: string]: number } = {
    'owner': 1,
    'admin': 2,
    'member': 3,
  };

  // Sort members based on the defined role order
  const sortedMembers = [...members].sort((a, b) => {
    const roleA = a.role.toLowerCase();
    const roleB = b.role.toLowerCase();
    const orderA = roleOrder[roleA] || 99; // Fallback for any unknown roles
    const orderB = roleOrder[roleB] || 99;
    
    if (orderA !== orderB) {
        return orderA - orderB;
    }
    // If roles are the same, sort by username
    return a.username.localeCompare(b.username);
  });

  const ownerCount = members.filter(m => m.role.toLowerCase() === 'owner').length;

  return (
    <div className="border border-slate-700 rounded-lg">
      <ul className="divide-y divide-slate-700">
        {sortedMembers.map(member => (
          <MemberListItem 
            key={member.id} 
            member={member} 
            isLastOwner={ownerCount === 1 && member.role.toLowerCase() === 'owner'}
            currentUser={currentUser}
            currentUserRole={currentUserRole}
            orgId={orgId}
            token={token}
            onDataChange={onDataChange}
          />
        ))}
      </ul>
    </div>
  );
};

export default MemberList;