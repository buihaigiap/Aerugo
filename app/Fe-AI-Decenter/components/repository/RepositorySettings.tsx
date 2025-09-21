import React, { useState } from 'react';
import { deleteRepository } from '../../services/api';
import { Repository } from '../../types';
import Button from '../Button';
import Modal from '../Modal';
import { TrashIcon } from '../icons/TrashIcon';
import Input from '../Input';

interface RepositorySettingsProps {
  token: string;
  organizationName: string;
  repository: Repository;
  onRepositoryDeleted: () => void;
}

const RepositorySettings: React.FC<RepositorySettingsProps> = ({ 
    token, 
    organizationName,
    repository, 
    onRepositoryDeleted
}) => {
  const [isDeleteModalOpen, setIsDeleteModalOpen] = useState(false);
  const [deleteConfirmationName, setDeleteConfirmationName] = useState('');
  const [isDeleting, setIsDeleting] = useState(false);
  const [deleteError, setDeleteError] = useState<string | null>(null);

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
    <div className="space-y-12 animate-fade-in">
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

export default RepositorySettings;
