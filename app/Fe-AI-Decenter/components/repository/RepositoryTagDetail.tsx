import React, { useState } from 'react';
import { ImageTag } from '../../types';
import { ArrowLeftIcon } from '../icons/ArrowLeftIcon';
import { ClipboardIcon } from '../icons/ClipboardIcon';
import { ClockIcon } from '../icons/ClockIcon';
import { CubeIcon } from '../icons/CubeIcon';
import Button from '../Button';
import Modal from '../Modal';
import { TrashIcon } from '../icons/TrashIcon';

interface RepositoryTagDetailProps {
  tag: ImageTag;
  repositoryPath: string;
  onBack: () => void;
}

const RepositoryTagDetail: React.FC<RepositoryTagDetailProps> = ({ tag, repositoryPath, onBack }) => {
  const [copyStatus, setCopyStatus] = useState('Copy');
  const [isDeleteModalOpen, setIsDeleteModalOpen] = useState(false);
  const [isDeleting, setIsDeleting] = useState(false);
  
  const pullCommand = `docker pull ${repositoryPath}@${tag.digest}`;

  const handleCopy = () => {
    navigator.clipboard.writeText(pullCommand).then(() => {
      setCopyStatus('Copied!');
      setTimeout(() => setCopyStatus('Copy'), 2000);
    }).catch(err => {
      console.error('Failed to copy text: ', err);
      setCopyStatus('Failed');
      setTimeout(() => setCopyStatus('Copy'), 2000);
    });
  };
  
  const handleDelete = () => {
    setIsDeleting(true);
    setTimeout(() => {
        setIsDeleting(false);
        setIsDeleteModalOpen(false);
        onBack();
    }, 1500);
  };

  return (
    <div className="space-y-8 animate-fade-in">
        <header>
            <button onClick={onBack} className="flex items-center text-sm text-blue-400 hover:text-blue-300 mb-4 focus:outline-none focus:ring-2 focus:ring-blue-500 rounded-md p-1 -ml-1">
                <ArrowLeftIcon className="w-4 h-4 mr-2" />
                Back to Tags
            </button>
            <h2 className="text-3xl font-bold text-slate-50">Tag: {tag.name}</h2>
        </header>

        {/* --- Main Info --- */}
        <div className="bg-slate-800/50 border border-slate-700 rounded-lg p-6 space-y-4">
            <div>
                <label className="block text-sm font-medium text-slate-400 mb-1">Digest</label>
                <p className="font-mono text-sm text-slate-200 break-all">{tag.digest}</p>
            </div>
             <div>
                <label className="block text-sm font-medium text-slate-400 mb-1">Created</label>
                <p className="text-sm text-slate-200">{new Date(tag.config.created).toLocaleString()}</p>
            </div>
            <div>
                <button
                    onClick={handleCopy}
                    className="inline-flex items-center px-4 py-2 bg-slate-700 hover:bg-slate-600 text-white font-semibold rounded-md shadow-sm transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 focus:ring-offset-slate-800"
                >
                    <ClipboardIcon className="w-5 h-5 mr-2" />
                    {copyStatus === 'Copy' ? 'Copy pull command' : copyStatus}
                </button>
            </div>
        </div>
        
        {/* --- Config & Labels --- */}
         <div className="bg-slate-800/50 border border-slate-700 rounded-lg p-6">
            <h3 className="text-xl font-semibold text-slate-200 mb-4">Configuration</h3>
            <dl className="grid grid-cols-1 sm:grid-cols-2 gap-x-6 gap-y-3 text-sm">
                <InfoPair label="Docker Version" value={tag.config.dockerVersion} />
                <InfoPair label="OS / Architecture" value={tag.config.osArch} />
                {Object.entries(tag.config.labels).map(([key, value]) => (
                    <InfoPair key={key} label={`Label: ${key}`} value={value} />
                ))}
            </dl>
        </div>

        {/* --- History --- */}
        <TableSection title="History" icon={<ClockIcon className="w-6 h-6 mr-3 text-slate-400" />}>
            <thead className="bg-slate-800/50">
                <tr>
                    <th scope="col" className="px-4 py-3 text-left text-xs font-medium text-slate-300 uppercase tracking-wider">Command</th>
                    <th scope="col" className="px-4 py-3 text-left text-xs font-medium text-slate-300 uppercase tracking-wider">Details</th>
                </tr>
            </thead>
            <tbody className="bg-slate-800 divide-y divide-slate-700">
                {tag.history.map((item, index) => (
                    <tr key={index}>
                        <td className="px-4 py-3 whitespace-nowrap text-sm font-mono font-bold text-slate-100">{item.command}</td>
                        <td className="px-4 py-3 text-sm text-slate-300 font-mono break-all">{item.details}</td>
                    </tr>
                ))}
            </tbody>
        </TableSection>
        
        {/* --- Layers --- */}
        <TableSection title="Layers" icon={<CubeIcon className="w-6 h-6 mr-3 text-slate-400" />}>
             <thead className="bg-slate-800/50">
                <tr>
                    <th scope="col" className="px-4 py-3 text-left text-xs font-medium text-slate-300 uppercase tracking-wider">Digest</th>
                    <th scope="col" className="px-4 py-3 text-left text-xs font-medium text-slate-300 uppercase tracking-wider">Size</th>
                </tr>
            </thead>
            <tbody className="bg-slate-800 divide-y divide-slate-700">
                {tag.layers.map((layer) => (
                    <tr key={layer.digest}>
                        <td className="px-4 py-3 whitespace-nowrap text-sm font-mono text-slate-300">{layer.digest}</td>
                        <td className="px-4 py-3 whitespace-nowrap text-sm text-slate-300">{layer.size}</td>
                    </tr>
                ))}
            </tbody>
        </TableSection>
        
        {/* --- Danger Zone --- */}
        <div className="bg-slate-800/50 border border-red-700/50 rounded-lg p-6">
          <h3 className="text-xl font-bold text-red-400 mb-2">Danger Zone</h3>
          <p className="text-slate-400 mb-4">Deleting a tag is permanent and cannot be undone unless the underlying image digest is referenced by another tag.</p>
          <Button variant="danger" onClick={() => setIsDeleteModalOpen(true)} fullWidth={false}>
            <TrashIcon className="w-5 h-5 mr-2 -ml-1" />
            Delete this tag
          </Button>
      </div>

       <Modal 
        isOpen={isDeleteModalOpen} 
        onClose={() => setIsDeleteModalOpen(false)}
        title="Delete Tag"
      >
        <div className="space-y-4">
            <p className="text-slate-300">
                Are you sure you want to delete the tag <strong className="font-bold text-slate-100">{tag.name}</strong>? 
                This action is irreversible.
            </p>
            <div className="flex justify-end items-center space-x-4 pt-2">
                <Button onClick={() => setIsDeleteModalOpen(false)} className="bg-transparent hover:bg-slate-700 text-slate-300" disabled={isDeleting}>
                    Cancel
                </Button>
                <Button 
                    variant="danger" 
                    onClick={handleDelete}
                    isLoading={isDeleting}
                >
                    Delete Tag
                </Button>
            </div>
        </div>
      </Modal>

    </div>
  );
};


const InfoPair: React.FC<{label: string, value: string}> = ({ label, value }) => (
    <>
        <dt className="font-medium text-slate-400">{label}</dt>
        <dd className="text-slate-200 break-words">{value}</dd>
    </>
);

const TableSection: React.FC<{ title: string; icon: React.ReactNode; children: React.ReactNode }> = ({ title, icon, children }) => (
    <div>
        <div className="flex items-center mb-4">
            {icon}
            <h3 className="text-xl font-semibold text-slate-200">{title}</h3>
        </div>
        <div className="overflow-x-auto">
            <div className="border border-slate-700 rounded-lg">
                <table className="min-w-full divide-y divide-slate-700">
                    {children}
                </table>
            </div>
        </div>
    </div>
);


export default RepositoryTagDetail;
