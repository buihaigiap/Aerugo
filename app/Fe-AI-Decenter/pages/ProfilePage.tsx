
import React, { useState } from 'react';
import { User, ChangePasswordRequest } from '../types';
import { changePassword } from '../services/api';
import { UserCircleIcon } from '../components/icons/UserCircleIcon';
import Input from '../components/Input';
import Button from '../components/Button';

interface ProfilePageProps {
  currentUser: User;
  token: string;
}

const ProfilePage: React.FC<ProfilePageProps> = ({ currentUser, token }) => {
  const [currentPassword, setCurrentPassword] = useState('');
  const [newPassword, setNewPassword] = useState('');
  const [confirmPassword, setConfirmPassword] = useState('');

  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState<string | null>(null);

  const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    setError(null);
    setSuccess(null);

    if (!currentPassword || !newPassword || !confirmPassword) {
      setError('Please fill in all fields.');
      return;
    }
    if (newPassword.length < 8) {
      setError('New password must be at least 8 characters long.');
      return;
    }
    if (newPassword !== confirmPassword) {
      setError('New passwords do not match.');
      return;
    }

    setIsLoading(true);
    try {
      const payload: ChangePasswordRequest = {
        current_password: currentPassword,
        new_password: newPassword,
        confirm_password: confirmPassword,
      };
      await changePassword(payload, token);
      setSuccess('Password updated successfully!');
      // Clear fields on success
      setCurrentPassword('');
      setNewPassword('');
      setConfirmPassword('');
    } catch (err: any) {
      if (err.status === 401) {
        setError('Incorrect current password.');
      } else {
        setError('An unexpected error occurred. Please try again.');
      }
      console.error(err);
    } finally {
      setIsLoading(false);
    }
  };


  return (
    <div className="space-y-8 animate-fade-in">
      <header>
        <h1 className="text-3xl font-bold text-slate-50">My Profile</h1>
        <p className="text-slate-400 mt-1">View and manage your account details.</p>
      </header>

      <div className="bg-slate-800 border border-slate-700 rounded-lg shadow-lg p-6 md:p-8">
        <div className="flex flex-col sm:flex-row items-center space-y-4 sm:space-y-0 sm:space-x-6">
          <div className="flex-shrink-0">
            <span className="h-24 w-24 rounded-full bg-slate-700 flex items-center justify-center">
              <UserCircleIcon className="h-16 w-16 text-slate-400" />
            </span>
          </div>
          <div>
            <h2 className="text-2xl font-bold text-slate-100">{currentUser.username}</h2>
            <p className="text-slate-400">{currentUser.email}</p>
          </div>
        </div>
      </div>
      
      <div className="bg-slate-800 border border-slate-700 rounded-lg shadow-lg">
        <div className="p-6">
            <h3 className="text-xl font-semibold text-slate-100">Account Information</h3>
            <p className="text-slate-400 mt-1">These are your account details. Profile editing is not yet available.</p>
        </div>
        <div className="border-t border-slate-700">
            <dl className="divide-y divide-slate-700">
                <div className="px-6 py-4 grid grid-cols-3 gap-4">
                    <dt className="text-sm font-medium text-slate-400">Username</dt>
                    <dd className="mt-1 text-sm text-slate-100 col-span-2 sm:mt-0">{currentUser.username}</dd>
                </div>
                <div className="px-6 py-4 grid grid-cols-3 gap-4">
                    <dt className="text-sm font-medium text-slate-400">Email address</dt>
                    <dd className="mt-1 text-sm text-slate-100 col-span-2 sm:mt-0">{currentUser.email}</dd>
                </div>
                 <div className="px-6 py-4 grid grid-cols-3 gap-4">
                    <dt className="text-sm font-medium text-slate-400">User ID</dt>
                    <dd className="mt-1 text-sm text-slate-100 col-span-2 sm:mt-0">{currentUser.id}</dd>
                </div>
            </dl>
        </div>
      </div>

       <div className="bg-slate-800 border border-slate-700 rounded-lg shadow-lg p-6">
            <h3 className="text-xl font-semibold text-slate-100">Change Password</h3>
            <form onSubmit={handleSubmit} className="mt-4 space-y-4 max-w-lg">
                <Input
                    id="currentPassword"
                    type="password"
                    label="Current Password"
                    value={currentPassword}
                    onChange={(e) => setCurrentPassword(e.target.value)}
                    disabled={isLoading}
                    autoComplete="current-password"
                />
                <Input
                    id="newPassword"
                    type="password"
                    label="New Password"
                    value={newPassword}
                    onChange={(e) => setNewPassword(e.target.value)}
                    disabled={isLoading}
                    autoComplete="new-password"
                />
                <Input
                    id="confirmPassword"
                    type="password"
                    label="Confirm New Password"
                    value={confirmPassword}
                    onChange={(e) => setConfirmPassword(e.target.value)}
                    disabled={isLoading}
                    autoComplete="new-password"
                />
                {error && <p className="text-sm text-red-500">{error}</p>}
                {success && <p className="text-sm text-green-500">{success}</p>}
                 <div className="flex justify-end pt-2">
                    <Button type="submit" isLoading={isLoading} fullWidth={false}>
                        Update Password
                    </Button>
                </div>
            </form>
      </div>

    </div>
  );
};

export default ProfilePage;