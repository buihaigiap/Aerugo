import React, { useState } from 'react';
import { OrganizationMember, User, OrganizationRole } from '../../types';
import { updateMemberRole, deleteMember } from '../../services/api';
import Button from '../Button';
import Modal from '../Modal';
import { TrashIcon } from '../icons/TrashIcon';

interface MemberListItemProps {
  member: OrganizationMember;
  isLastOwner: boolean;
  currentUser: User;
  currentUserRole?: string;
  orgId: number;
  token: string;
  onDataChange: () => void;
}

const MemberListItem: React.FC<MemberListItemProps> = ({ member, isLastOwner, currentUser, currentUserRole, orgId, token, onDataChange }) => {
  const [isUpdating, setIsUpdating] = useState(false);
  const [isDeleting, setIsDeleting] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [isDeleteModalOpen, setIsDeleteModalOpen] = useState(false);

  const handleRoleChange = async (e: React.ChangeEvent<HTMLSelectElement>) => {
    const newRoleValue = e.target.value;
    const capitalizedRole = (newRoleValue.charAt(0).toUpperCase() + newRoleValue.slice(1)) as OrganizationRole;

    setIsUpdating(true);
    setError(null);
    try {
      // The API requires the user's ID for updates, not the membership ID.
      await updateMemberRole(orgId, member.user_id, capitalizedRole, token);
      onDataChange();
    } catch (err) {
      console.error(err);
      setError("Failed to update role.");
    } finally {
      setIsUpdating(false);
    }
  };

  const handleConfirmDelete = async () => {
    setIsDeleting(true);
    setError(null);
    try {
      // The API for deleting a member requires the user's ID, not the membership ID.
      await deleteMember(orgId, member.user_id, token);
      setIsDeleteModalOpen(false);
      onDataChange();
    } catch (err) {
      console.error(err);
      setError("Failed to remove member.");
    } finally {
      setIsDeleting(false);
    }
  };
  
  const roleColors: { [key: string]: string } = {
    owner: 'bg-purple-900/70 text-purple-300',
    admin: 'bg-indigo-900/70 text-indigo-300',
    member: 'bg-slate-600/70 text-slate-300',
  };

  const roleClass = roleColors[member.role.toLowerCase()] || roleColors.member;
  const displayRole = member.role.charAt(0).toUpperCase() + member.role.slice(1);

  const canManage = currentUserRole === 'owner' || (currentUserRole === 'admin' && member.role.toLowerCase() !== 'owner');
  const isSelf = currentUser.id === member.user_id;

  return (
    <>
      <li className="p-4 flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4">
        <div className="flex-1">
          <p className="text-md font-semibold text-slate-100">{member.username}</p>
          <p className="text-sm text-slate-400">{member.email}</p>
          {error && !isDeleteModalOpen && <p className="text-xs text-red-500 mt-1">{error}</p>}
        </div>
        <div className="flex items-center space-x-4 w-full sm:w-auto justify-end">
          {canManage && !isSelf ? (
            <select
              value={member.role.toLowerCase()}
              onChange={handleRoleChange}
              disabled={isUpdating || isLastOwner}
              className="block w-full max-w-[150px] px-3 py-1 bg-slate-700 border border-slate-600 rounded-md text-slate-100 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm disabled:opacity-70"
            >
              <option value="owner" disabled={currentUserRole !== 'owner'}>Owner</option>
              <option value="admin">Admin</option>
              <option value="member">Member</option>
            </select>
          ) : (
            <span className={`inline-flex items-center px-3 py-1 rounded-full text-xs font-medium ${roleClass}`}>
              {displayRole}
            </span>
          )}
          
          {canManage && !isSelf && !isLastOwner && (
              <Button 
                  variant='danger' 
                  onClick={() => {
                    setError(null);
                    setIsDeleteModalOpen(true);
                  }}
                  className="!w-auto !p-2"
                  title={`Remove ${member.username}`}
              >
                  <TrashIcon className="w-4 h-4" />
              </Button>
          )}
        </div>
      </li>

      <Modal 
        isOpen={isDeleteModalOpen} 
        onClose={() => setIsDeleteModalOpen(false)}
        title="Remove Member"
      >
        <div className="space-y-4">
            <p className="text-slate-300">
                Are you sure you want to remove <strong className="font-bold text-slate-100">{member.username}</strong> from the organization?
            </p>
            {error && <p className="text-sm text-red-500">{error}</p>}
            <div className="flex justify-end items-center space-x-4 pt-2">
                <Button onClick={() => setIsDeleteModalOpen(false)} className="bg-transparent hover:bg-slate-700 text-slate-300" disabled={isDeleting}>
                    Cancel
                </Button>
                <Button 
                    variant="danger" 
                    onClick={handleConfirmDelete}
                    isLoading={isDeleting}
                >
                    Remove Member
                </Button>
            </div>
        </div>
      </Modal>
    </>
  );
};

export default MemberListItem;