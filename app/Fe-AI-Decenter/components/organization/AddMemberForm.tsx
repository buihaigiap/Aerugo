import React, { useState } from 'react';
import { addOrganizationMember } from '../../services/api';
import { OrganizationRole, AddMemberRequest } from '../../types';
import Input from '../Input';
import Button from '../Button';
import { ShieldCheckIcon } from '../icons/ShieldCheckIcon';
import { UsersIcon } from '../icons/UsersIcon';

interface AddMemberFormProps {
  token: string;
  orgId: number;
  onSuccess: () => void;
  onCancel: () => void;
}

const roleOptions = [
  { 
    value: OrganizationRole.Member, 
    title: "Member", 
    description: "Can view repositories and pull images.",
    icon: <UsersIcon className="w-5 h-5 mb-2" /> 
  },
  { 
    value: OrganizationRole.Admin, 
    title: "Admin", 
    description: "Can manage repositories and members.",
    icon: <ShieldCheckIcon className="w-5 h-5 mb-2" />
  },
  { 
    value: OrganizationRole.Owner, 
    title: "Owner", 
    description: "Full administrative access to the organization.",
    icon: <ShieldCheckIcon className="w-5 h-5 mb-2 text-purple-400" />
  },
];

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
    <div className="bg-slate-800/50 border border-slate-700 rounded-lg p-6 animate-fade-in-up">
      <h3 className="text-lg font-bold text-slate-50 mb-6">Add New Member</h3>
      <form onSubmit={handleSubmit} className="space-y-6">
        <Input
          id="email"
          name="email"
          type="email"
          label="Email Address"
          value={email}
          onChange={(e) => setEmail(e.target.value)}
          placeholder="user@example.com"
          required
          autoFocus
        />
        
        <div>
            <label className="block text-sm font-medium text-slate-300 mb-2">
                Role
            </label>
            <div className="grid grid-cols-1 sm:grid-cols-3 gap-4">
                {roleOptions.map((option) => (
                    <RoleSelectionCard
                        key={option.value}
                        id={`role-${option.value}`}
                        icon={option.icon}
                        title={option.title}
                        description={option.description}
                        isSelected={role === option.value}
                        onSelect={() => setRole(option.value)}
                    />
                ))}
            </div>
        </div>

        {error && <p className="text-sm text-red-500">{error}</p>}
        
        <div className="flex justify-between items-center mt-6 pt-5 border-t border-slate-700/60">
            <Button 
              type="button" 
              onClick={onCancel} 
              fullWidth={false}
              className="bg-transparent hover:bg-slate-700 border-slate-700 hover:border-slate-600 text-slate-300"
              disabled={isLoading}
            >
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

interface RoleSelectionCardProps {
    id: string;
    icon: React.ReactNode;
    title: string;
    description: string;
    isSelected: boolean;
    onSelect: () => void;
}

const RoleSelectionCard: React.FC<RoleSelectionCardProps> = ({ id, icon, title, description, isSelected, onSelect }) => {
    return (
         <label 
            htmlFor={id} 
            className={`relative flex flex-col text-center p-4 border rounded-lg cursor-pointer transition-all duration-200 
                ${isSelected 
                    ? 'bg-indigo-900/50 border-indigo-500 scale-105 shadow-lg' 
                    : 'bg-slate-700/80 border-slate-600 hover:border-slate-500'
                }`
            }
        >
            <input type="radio" id={id} name="role" checked={isSelected} onChange={onSelect} className="hidden" />
            <div className={`mx-auto ${isSelected ? 'text-indigo-300' : 'text-slate-400'}`}>{icon}</div>
            <span className="block text-md font-semibold text-slate-100">{title}</span>
            <span className="block text-xs text-slate-400 mt-1">{description}</span>
        </label>
    );
}

export default AddMemberForm;