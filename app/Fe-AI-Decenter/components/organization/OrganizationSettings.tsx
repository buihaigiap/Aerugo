import React, { useState } from 'react';
import { updateOrganization, deleteOrganization } from '../../services/api';
import { Organization, UpdateOrganizationRequest } from '../../types';
import Input from '../Input';
import Button from '../Button';
import Modal from '../Modal';
import { TrashIcon } from '../icons/TrashIcon';

interface OrganizationSettingsProps {
  token: string;
  organization: Organization;
  onOrganizationUpdated: () => void;
  onOrganizationDeleted: () => void;
}

const OrganizationSettings: React.FC<OrganizationSettingsProps> = ({ 
    token, 
    organization, 
    onOrganizationUpdated,
    onOrganizationDeleted
}) => {
  const [formData, setFormData] = useState<UpdateOrganizationRequest>({
    display_name: organization.display_name,
    description: organization.description || '',
    avatar_url: organization.avatar_url || '',
    website_url: organization.website_url || '',
  });
  const [isUpdating, setIsUpdating] = useState(false);
  const [updateError, setUpdateError] = useState<string | null>(null);
  const [updateSuccess, setUpdateSuccess] = useState<string | null>(null);

  const [isDeleteModalOpen, setIsDeleteModalOpen] = useState(false);
  const [deleteConfirmationName, setDeleteConfirmationName] = useState('');
  const [isDeleting, setIsDeleting] = useState(false);
  const [deleteError, setDeleteError] = useState<string | null>(null);

  const handleChange = (e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) => {
    const { name, value } = e.target;
    setFormData(prev => ({ ...prev, [name]: value }));
  };

  const handleUpdateSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    if (!formData.display_name) {
      setUpdateError('Display name is required.');
      return;
    }
    
    setIsUpdating(true);
    setUpdateError(null);
    setUpdateSuccess(null);
    try {
      const payload: UpdateOrganizationRequest = {
          ...formData,
          avatar_url: formData.avatar_url || undefined,
          website_url: formData.website_url || undefined,
      };
      await updateOrganization(organization.id, payload, token);
      setUpdateSuccess('Organization updated successfully!');
      onOrganizationUpdated();
      setTimeout(() => setUpdateSuccess(null), 3000);
    } catch (err) {
      setUpdateError('Failed to update organization. Please try again.');
      console.error(err);
    } finally {
      setIsUpdating(false);
    }
  };

  const handleDelete = async () => {
    if (deleteConfirmationName !== organization.name) {
        setDeleteError(`Type "${organization.name}" to confirm.`);
        return;
    }
    setIsDeleting(true);
    setDeleteError(null);
    try {
        await deleteOrganization(organization.id, token);
        setIsDeleteModalOpen(false);
        onOrganizationDeleted();
    } catch (err) {
        setDeleteError('Failed to delete organization. Please try again.');
        console.error(err);
    } finally {
        setIsDeleting(false);
    }
  };

  return (
    <div className="space-y-12">
      <div className="bg-slate-800/50 border border-slate-700 rounded-lg p-6">
        <h3 className="text-xl font-bold text-slate-50 mb-4">General Settings</h3>
        <form onSubmit={handleUpdateSubmit} className="space-y-4">
          <div>
            <Input
              id="display_name"
              name="display_name"
              label="Display Name"
              value={formData.display_name}
              onChange={handleChange}
              placeholder="My Awesome Team"
              required
            />
          </div>
          <div>
            <label className="block text-sm font-medium text-slate-300 mb-2">Organization Name (URL)</label>
            <p className="px-4 py-2 bg-slate-700/50 border border-slate-600 rounded-md text-slate-400 sm:text-sm">{organization.name}</p>
          </div>
          <div>
            <Input
              id="description"
              name="description"
              label="Description (Optional)"
              value={formData.description || ''}
              onChange={handleChange}
              placeholder="A brief description of your organization."
            />
          </div>
           <div>
            <Input
              id="website_url"
              name="website_url"
              label="Website URL (Optional)"
              type="url"
              value={formData.website_url || ''}
              onChange={handleChange}
              placeholder="https://example.com"
            />
          </div>
          <div>
            <Input
              id="avatar_url"
              name="avatar_url"
              label="Avatar URL (Optional)"
              type="url"
              value={formData.avatar_url || ''}
              onChange={handleChange}
              placeholder="https://example.com/logo.png"
            />
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
          <p className="text-slate-400 mb-4">Deleting an organization is permanent and cannot be undone. This will also delete all associated repositories and images.</p>
          <Button variant="danger" onClick={() => setIsDeleteModalOpen(true)} fullWidth={false}>
            <TrashIcon className="w-5 h-5 mr-2 -ml-1" />
            Delete this organization
          </Button>
      </div>
      
      <Modal 
        isOpen={isDeleteModalOpen} 
        onClose={() => setIsDeleteModalOpen(false)}
        title="Delete Organization"
      >
        <div className="space-y-4">
            <p className="text-slate-300">
                Are you sure you want to delete <strong className="font-bold text-slate-100">{organization.display_name}</strong>? 
                This action is irreversible.
            </p>
            <p className="text-slate-400 text-sm">To confirm, please type <strong className="font-mono text-red-400">{organization.name}</strong> in the box below.</p>
            <Input 
                id="delete-confirm"
                label=""
                value={deleteConfirmationName}
                onChange={(e) => setDeleteConfirmationName(e.target.value)}
                placeholder={organization.name}
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
                    disabled={deleteConfirmationName !== organization.name || isDeleting}
                >
                    Delete Organization
                </Button>
            </div>
        </div>
      </Modal>

    </div>
  );
};

export default OrganizationSettings;