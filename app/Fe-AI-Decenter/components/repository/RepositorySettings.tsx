import React, { useState } from 'react';
import { deleteRepository, updateRepository } from '../../services/api';
import { Repository, UpdateRepositoryRequest } from '../../types';
import Button from '../Button';
import Modal from '../Modal';
import { TrashIcon } from '../icons/TrashIcon';
import Input from '../Input';
import { GlobeIcon } from '../icons/GlobeIcon';
import { LockIcon } from '../icons/LockIcon';

interface RepositorySettingsProps {
  token: string;
  organizationName: string;
  repository: Repository;
  onRepositoryDeleted: () => void;
  onRepositoryUpdated: () => void;
}

const RepositorySettings: React.FC<RepositorySettingsProps> = ({ 
    token, 
    organizationName,
    repository, 
    onRepositoryDeleted,
    onRepositoryUpdated
}) => {
  const [formData, setFormData] = useState({
    name: repository.name,
    description: repository.description || '',
    is_public: repository.is_public,
  });
  const [isUpdating, setIsUpdating] = useState(false);
  const [updateError, setUpdateError] = useState<string | null>(null);
  const [updateSuccess, setUpdateSuccess] = useState<string | null>(null);

  const [isDeleteModalOpen, setIsDeleteModalOpen] = useState(false);
  const [deleteConfirmationName, setDeleteConfirmationName] = useState('');
  const [isDeleting, setIsDeleting] = useState(false);
  const [deleteError, setDeleteError] = useState<string | null>(null);
  
  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value } = e.target;
    setFormData(prev => ({ ...prev, [name]: value }));
  };

  const handleVisibilityChange = (isPublic: boolean) => {
    setFormData(prev => ({ ...prev, is_public: isPublic }));
  };

  const handleUpdateSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    setUpdateError(null);
    setUpdateSuccess(null);
    
    if (!formData.name) {
      setUpdateError('Repository name is required.');
      return;
    }
    
    if (!/^[a-z0-9]+(?:[._-][a-z0-9]+)*$/.test(formData.name)) {
        setUpdateError('Name must be lowercase, alphanumeric, and can contain separators (-, _, .).');
        return;
    }

    setIsUpdating(true);
    try {
      const apiPayload: UpdateRepositoryRequest = {
        name: formData.name,
        description: formData.description || null,
        is_public: formData.is_public,
      };
      
      // We use the original repository name to identify it in the API call
      await updateRepository(organizationName, repository.name, apiPayload, token);
      setUpdateSuccess('Repository updated successfully!');
      onRepositoryUpdated();
      setTimeout(() => setUpdateSuccess(null), 3000);
    } catch (err) {
      setUpdateError('Failed to update repository. The new name might be taken.');
      console.error(err);
    } finally {
      setIsUpdating(false);
    }
  };


  const handleDelete = async () => {
    if (deleteConfirmationName !== repository.name) {
        setDeleteError(`Type "${repository.name}" to confirm.`);
        return;
    }
    setIsDeleting(true);
    setDeleteError(null);
    try {
        await deleteRepository(organizationName, repository.name, token);
        setIsDeleteModalOpen(false);
        onRepositoryDeleted();
    } catch (err) {
        setDeleteError('Failed to delete repository. Please try again.');
        console.error(err);
    } finally {
        setIsDeleting(false);
    }
  };

  return (
    <div className="space-y-12 animate-fade-in-up">
        <div className="bg-slate-800/50 border border-slate-700 rounded-lg p-6">
            <h3 className="text-xl font-bold text-slate-50 mb-6">General Settings</h3>
            <form onSubmit={handleUpdateSubmit} className="space-y-6">
                 <div>
                    <Input
                        id="name"
                        name="name"
                        label="Repository Name"
                        value={formData.name}
                        onChange={handleChange}
                        required
                    />
                    <p className="text-xs text-slate-400 mt-2 ml-1">Lowercase, numbers, and separators (-, _, .) only.</p>
                </div>

                <Input
                    id="description"
                    name="description"
                    label="Description (Optional)"
                    value={formData.description}
                    onChange={handleChange}
                />

                <div>
                    <label className="block text-sm font-medium text-slate-300 mb-2">Visibility</label>
                    <div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
                        <VisibilityOption
                            id="visibility-public-update"
                            icon={<GlobeIcon className="w-5 h-5 mr-3 text-green-400"/>}
                            label="Public"
                            description="Anyone can see this repository."
                            isSelected={formData.is_public}
                            onSelect={() => handleVisibilityChange(true)}
                        />
                        <VisibilityOption
                            id="visibility-private-update"
                            icon={<LockIcon className="w-5 h-5 mr-3 text-yellow-400"/>}
                            label="Private"
                            description="You choose who can see this repository."
                            isSelected={!formData.is_public}
                            onSelect={() => handleVisibilityChange(false)}
                        />
                    </div>
                </div>

                {updateError && <p className="text-sm text-red-500">{updateError}</p>}
                {updateSuccess && <p className="text-sm text-green-500">{updateSuccess}</p>}

                <div className="flex justify-end items-center pt-2">
                    <Button type="submit" isLoading={isUpdating} fullWidth={false}>
                        Save Changes
                    </Button>
                </div>
            </form>
        </div>


      <div className="bg-slate-800/50 border border-red-700/50 rounded-lg p-6">
          <h3 className="text-xl font-bold text-red-400 mb-2">Danger Zone</h3>
          <p className="text-slate-400 mb-4">Deleting a repository is permanent and cannot be undone. This will delete all associated tags and images.</p>
          <Button variant="danger" onClick={() => setIsDeleteModalOpen(true)} fullWidth={false}>
            <TrashIcon className="w-5 h-5 mr-2 -ml-1" />
            Delete this repository
          </Button>
      </div>
      
      <Modal 
        isOpen={isDeleteModalOpen} 
        onClose={() => setIsDeleteModalOpen(false)}
        title="Delete Repository"
      >
        <div className="space-y-4">
            <p className="text-slate-300">
                Are you sure you want to delete the repository <strong className="font-bold text-slate-100">{repository.name}</strong>? 
                This action is irreversible.
            </p>
            <p className="text-slate-400 text-sm">To confirm, please type <strong className="font-mono text-red-400">{repository.name}</strong> in the box below.</p>
            <Input 
                id="delete-confirm"
                label=""
                value={deleteConfirmationName}
                onChange={(e) => setDeleteConfirmationName(e.target.value)}
                placeholder={repository.name}
            />
             {deleteError && <p className="text-sm text-red-500">{deleteError}</p>}
            <div className="flex justify-end items-center space-x-4 pt-2">
                <Button onClick={() => setIsDeleteModalOpen(false)} className="bg-transparent hover:bg-slate-700 text-slate-300">
                    Cancel
                </Button>
                <Button 
                    variant="danger" 
                    onClick={handleDelete}
                    isLoading={isDeleting}
                    disabled={deleteConfirmationName !== repository.name || isDeleting}
                >
                    Delete Repository
                </Button>
            </div>
        </div>
      </Modal>
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
            <input type="radio" id={id} name="visibility-update" checked={isSelected} onChange={onSelect} className="hidden" />
            {icon}
            <div className="flex-1">
                <span className="block text-md font-semibold text-slate-100">{label}</span>
                <span className="block text-sm text-slate-400">{description}</span>
            </div>
        </label>
    );
}


export default RepositorySettings;