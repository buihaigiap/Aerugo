import React, { useState } from 'react';
import { createRepository } from '../../services/api';
import { CreateRepositoryRequest, Repository, Organization } from '../../types';
import Input from '../Input';
import Button from '../Button';
import { GlobeIcon } from '../icons/GlobeIcon';
import { LockIcon } from '../icons/LockIcon';
import { ServerStackIcon } from '../icons/ServerStackIcon';

interface CreateRepositoryFormProps {
  token: string;
  organization: Organization;
  onSuccess: (newRepo: Repository) => void;
  onCancel: () => void;
}

// Define an interface for the form's internal state for better type safety
interface RepositoryFormState {
  name: string;
  description: string;
  visibility: 'public' | 'private';
}

const CreateRepositoryForm: React.FC<CreateRepositoryFormProps> = ({ token, organization, onSuccess, onCancel }) => {
  const [formData, setFormData] = useState<RepositoryFormState>({
    name: '',
    description: '',
    visibility: 'private',
  });
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value } = e.target;
    setFormData(prev => ({ ...prev, [name]: value }));
  };

  const handleVisibilityChange = (visibility: 'public' | 'private') => {
    setFormData(prev => ({ ...prev, visibility }));
  }

  const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    setError(null);
    
    if (!formData.name) {
      setError('Repository name is required.');
      return;
    }
    
    // Basic validation for repo name (URL-friendly)
    if (!/^[a-z0-9]+(?:[._-][a-z0-9]+)*$/.test(formData.name)) {
        setError('Name must be lowercase, alphanumeric, and can contain separators (-, _, .). It cannot start or end with a separator.');
        return;
    }

    setIsLoading(true);
    try {
      // Transform the form state into the required API payload
      const apiPayload: CreateRepositoryRequest = {
        name: formData.name,
        description: formData.description || null, // Send null if description is empty
        is_public: formData.visibility === 'public',
      };
      
      const newRepo = await createRepository(organization.name, apiPayload, token);
      onSuccess(newRepo);
    } catch (err) {
      setError('Failed to create repository. The name might already be taken.');
      console.error(err);
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="bg-slate-800/50 border border-slate-700 rounded-lg p-6 animate-fade-in-up">
        <div className="flex items-start gap-4 mb-8">
            <div className="flex-shrink-0 p-3 bg-slate-700/50 rounded-lg border border-slate-600">
                <ServerStackIcon className="w-6 h-6 text-indigo-400" />
            </div>
            <div>
                <h3 className="text-xl font-bold text-slate-50">Create New Repository</h3>
                <p className="text-slate-400 text-sm mt-1">
                  Repositories are namespaces for your images within the <strong className="text-slate-300 font-semibold">{organization.name}</strong> organization.
                </p>
            </div>
        </div>

      <form onSubmit={handleSubmit} className="space-y-6">
        <div>
            <Input
              id="name"
              name="name"
              label="Repository Name"
              value={formData.name}
              onChange={handleChange}
              placeholder="my-awesome-app"
              required
              autoFocus
            />
            <p className="text-xs text-slate-400 mt-2 ml-1">Lowercase, numbers, and separators (-, _, .) only.</p>
        </div>

        <Input
          id="description"
          name="description"
          label="Description (Optional)"
          value={formData.description || ''}
          onChange={handleChange}
          placeholder="A brief description of this repository."
        />

        <div>
            <label className="block text-sm font-medium text-slate-300 mb-2">Visibility</label>
            <div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
                <VisibilityOption
                    id="visibility-public"
                    icon={<GlobeIcon className="w-5 h-5 mr-3 text-green-400"/>}
                    label="Public"
                    description="Anyone can see this repository."
                    isSelected={formData.visibility === 'public'}
                    onSelect={() => handleVisibilityChange('public')}
                />
                <VisibilityOption
                    id="visibility-private"
                    icon={<LockIcon className="w-5 h-5 mr-3 text-yellow-400"/>}
                    label="Private"
                    description="You choose who can see this repository."
                    isSelected={formData.visibility === 'private'}
                    onSelect={() => handleVisibilityChange('private')}
                />
            </div>
        </div>


        {error && <p className="text-sm text-red-500 text-center">{error}</p>}
        
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
                Create Repository
            </Button>
        </div>
      </form>
    </div>
  );
};


interface VisibilityOptionProps {
    id: string;
    icon: React.ReactNode;
    label: string;
    description: string;
    isSelected: boolean;
    onSelect: () => void;
}

const VisibilityOption: React.FC<VisibilityOptionProps> = ({ id, icon, label, description, isSelected, onSelect }) => {
    return (
         <label htmlFor={id} className={`relative flex items-start p-4 border rounded-lg cursor-pointer transition-colors duration-200 ${isSelected ? 'bg-indigo-900/50 border-indigo-500' : 'bg-slate-700/80 border-slate-600 hover:border-slate-500'}`}>
            <input type="radio" id={id} name="visibility-create" checked={isSelected} onChange={onSelect} className="hidden" />
            {icon}
            <div className="flex-1">
                <span className="block text-md font-semibold text-slate-100">{label}</span>
                <span className="block text-sm text-slate-400">{description}</span>
            </div>
        </label>
    );
}

export default CreateRepositoryForm;