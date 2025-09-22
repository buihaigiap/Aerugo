import React, { useState } from 'react';
import { addOrganizationMember } from '../../services/api';
import { OrganizationRole, AddMemberRequest } from '../../types';
import Input from '../Input';
import Button from '../Button';

interface AddMemberFormProps {
  token: string;
  orgId: number;
  onSuccess: () => void;
  onCancel: () => void;
}

const AddMemberForm: React.FC<AddMemberFormProps> = ({ token, orgId, onSuccess, onCancel }) => {
  const [email, setEmail] = useState('');
  const [role, setRole] = useState<OrganizationRole>(OrganizationRole.Member);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    if (!email) {
      setError('Email address is required.');
      return;
    }
    
    setIsLoading(true);
    setError(null);
    try {
      const data: AddMemberRequest = { email, role };
      await addOrganizationMember(orgId, data, token);
      onSuccess();
    } catch (err: any) {
        if (err.status === 400) {
            setError('User with that email not found.');
        } else if (err.status === 409) {
            setError('User is already a member of this organization.');
        } else {
            setError('Failed to add member. Please try again.');
        }
      console.error(err);
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="bg-slate-800/50 border border-slate-700 rounded-lg p-6">
      <h3 className="text-lg font-bold text-slate-50 mb-4">Add New Member</h3>
      <form onSubmit={handleSubmit} className="space-y-4">
        <Input
          id="email"
          name="email"
          type="email"
          label="Email Address"
          value={email}
          onChange={(e) => setEmail(e.target.value)}
          placeholder="user@example.com"
          required
        />
        
        <div>
            <label htmlFor="role" className="block text-sm font-medium text-slate-300 mb-2">
                Role
            </label>
            <select
                id="role"
                name="role"
                value={role}
                onChange={(e) => setRole(e.target.value as OrganizationRole)}
                className="block w-full px-4 py-2 bg-slate-700 border border-slate-600 rounded-md text-slate-100 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 sm:text-sm"
            >
                <option value={OrganizationRole.Owner}>Owner</option>
                <option value={OrganizationRole.Admin}>Admin</option>
                <option value={OrganizationRole.Member}>Member</option>
            </select>
        </div>

        {error && <p className="text-sm text-red-500">{error}</p>}
        
        <div className="flex justify-end items-center space-x-4 pt-2">
            <Button type="button" onClick={onCancel} className="bg-transparent hover:bg-slate-700 text-slate-300">
                Cancel
            </Button>
            <Button type="submit" isLoading={isLoading} fullWidth={false}>
                Add Member
            </Button>
        </div>
      </form>
    </div>
  );
};

export default AddMemberForm;