import React, { useState } from 'react';
import { User, ChangePasswordRequest } from '../types';
import { changePassword } from '../services/api';
import { UserCircleIcon } from '../components/icons/UserCircleIcon';
import { IdentificationIcon } from '../components/icons/IdentificationIcon';
import { KeyIcon } from '../components/icons/KeyIcon';
import Input from '../components/Input';
import Button from '../components/Button';

interface ProfilePageProps {
  currentUser: User;
  token: string;
}

type Tab = 'details' | 'password';

const ProfilePage: React.FC<ProfilePageProps> = ({ currentUser, token }) => {
  const [activeTab, setActiveTab] = useState<Tab>('details');

  return (
    <div className="space-y-8 animate-fade-in-up">
      <header>
        <h1 className="text-3xl font-bold text-slate-50">My Profile</h1>
        <p className="text-slate-400 mt-1">View your account details and manage your password.</p>
      </header>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-8">
        {/* Left Column: User Profile Card */}
        <aside className="lg:col-span-1">
          <div className="bg-slate-800/50 border border-slate-700/80 rounded-xl p-8 text-center shadow-lg h-full flex flex-col justify-center">
            <span className="h-32 w-32 rounded-full bg-slate-700 flex items-center justify-center mx-auto border-4 border-slate-600">
              <UserCircleIcon className="h-24 w-24 text-indigo-400" />
            </span>
            <h2 className="mt-6 text-2xl font-bold text-slate-100 truncate">{currentUser.username}</h2>
            <p className="mt-1 text-slate-400 truncate">{currentUser.email}</p>
            <div className="mt-4 inline-flex items-center px-3 py-1 bg-slate-700 rounded-full text-xs font-medium text-slate-300">
              User ID: {currentUser.id}
            </div>
          </div>
        </aside>

        {/* Right Column: Tabbed Content */}
        <main className="lg:col-span-2">
          <div className="bg-slate-800 border border-slate-700 rounded-xl shadow-lg h-full">
            <div className="border-b border-slate-700">
              <nav className="flex space-x-1 p-2" aria-label="Tabs">
                <TabButton
                  icon={<IdentificationIcon className="w-5 h-5 mr-2" />}
                  label="Account Details"
                  isActive={activeTab === 'details'}
                  onClick={() => setActiveTab('details')}
                />
                <TabButton
                  icon={<KeyIcon className="w-5 h-5 mr-2" />}
                  label="Security"
                  isActive={activeTab === 'password'}
                  onClick={() => setActiveTab('password')}
                />
              </nav>
            </div>
            
            <div className="p-6 md:p-8">
              {activeTab === 'details' && <AccountDetails currentUser={currentUser} />}
              {activeTab === 'password' && <ChangePasswordForm token={token} />}
            </div>
          </div>
        </main>
      </div>
    </div>
  );
};

const TabButton: React.FC<{ icon: React.ReactNode, label: string, isActive: boolean, onClick: () => void }> = ({ icon, label, isActive, onClick }) => (
  <button
    onClick={onClick}
    className={`flex-1 flex items-center justify-center whitespace-nowrap py-2.5 px-4 font-medium text-sm transition-colors duration-200 rounded-lg focus:outline-none focus-visible:ring-2 focus-visible:ring-indigo-500 focus-visible:ring-offset-2 focus-visible:ring-offset-slate-800
      ${
        isActive
          ? 'bg-slate-700 text-slate-50'
          : 'text-slate-400 hover:text-slate-200 hover:bg-slate-700/50'
      }`}
  >
    {icon}
    <span>{label}</span>
  </button>
);

const AccountDetails: React.FC<{ currentUser: User }> = ({ currentUser }) => (
  <div className="animate-fade-in-up">
    <h3 className="text-xl font-semibold text-slate-100">Account Information</h3>
    <p className="text-slate-400 mt-1 mb-6">These are your account details. Profile editing is not yet available.</p>
    <dl className="space-y-4">
      <div className="flex flex-col sm:flex-row sm:items-center">
        <dt className="w-full sm:w-1/3 text-sm font-medium text-slate-400">Username</dt>
        <dd className="w-full sm:w-2/3 mt-1 sm:mt-0 text-sm text-slate-100 bg-slate-700/50 border border-slate-600 rounded-md px-4 py-2">{currentUser.username}</dd>
      </div>
      <div className="flex flex-col sm:flex-row sm:items-center">
        <dt className="w-full sm:w-1/3 text-sm font-medium text-slate-400">Email address</dt>
        <dd className="w-full sm:w-2/3 mt-1 sm:mt-0 text-sm text-slate-100 bg-slate-700/50 border border-slate-600 rounded-md px-4 py-2">{currentUser.email}</dd>
      </div>
      <div className="flex flex-col sm:flex-row sm:items-center">
        <dt className="w-full sm:w-1/3 text-sm font-medium text-slate-400">User ID</dt>
        <dd className="w-full sm:w-2/3 mt-1 sm:mt-0 text-sm text-slate-100 bg-slate-700/50 border border-slate-600 rounded-md px-4 py-2">{currentUser.id}</dd>
      </div>
    </dl>
  </div>
);

const ChangePasswordForm: React.FC<{ token: string }> = ({ token }) => {
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
    <div className="animate-fade-in-up">
      <h3 className="text-xl font-semibold text-slate-100">Change Password</h3>
      <form onSubmit={handleSubmit} className="mt-4 space-y-4">
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
  );
};

export default ProfilePage;