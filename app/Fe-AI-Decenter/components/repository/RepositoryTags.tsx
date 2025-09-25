import React, { useState } from 'react';
import { ImageTag } from '../../types';
import { ClipboardIcon } from '../icons/ClipboardIcon';
import { TagIcon } from '../icons/TagIcon';

interface RepositoryTagsProps {
    repositoryPath: string;
    tags: ImageTag[];
    isLoading: boolean;
    error: string | null;
}

const RepositoryTags: React.FC<RepositoryTagsProps> = ({ repositoryPath, tags, isLoading, error }) => {
    // NOTE: The tag detail view has been temporarily disabled as the API does not yet provide detailed tag information.
    // The state and logic for 'selectedTag' have been removed.

    if (isLoading) {
        return (
            <div className="text-center py-10 px-4">
                <p className="text-slate-400">Loading tags...</p>
            </div>
        );
    }

    if (error) {
        return (
            <div className="text-center py-10 px-4 border-2 border-dashed border-red-700/50 bg-red-900/20 rounded-lg">
                <h3 className="text-lg font-medium text-red-300">Error Loading Tags</h3>
                <p className="text-red-400 mt-1">{error}</p>
            </div>
        );
    }
    
    if (!tags || tags.length === 0) {
        return (
            <div className="text-center py-10 px-4 border-2 border-dashed border-slate-700 rounded-lg">
                <TagIcon className="mx-auto h-12 w-12 text-slate-500" />
                <h3 className="mt-2 text-lg font-medium text-slate-300">No Tags Found</h3>
                <p className="text-slate-400 mt-1">This repository is empty. Push an image to see its tags here.</p>
            </div>
        );
    }

    return (
        <div className="overflow-x-auto animate-fade-in-up">
            <div className="border border-slate-700 rounded-lg">
                <table className="min-w-full divide-y divide-slate-700">
                    <thead className="bg-slate-800/50">
                        <tr>
                            <th scope="col" className="px-6 py-3 text-left text-xs font-medium text-slate-300 uppercase tracking-wider">Tag</th>
                            <th scope="col" className="px-6 py-3 text-left text-xs font-medium text-slate-300 uppercase tracking-wider">Digest</th>
                            <th scope="col" className="px-6 py-3 text-left text-xs font-medium text-slate-300 uppercase tracking-wider">OS/Arch</th>
                            <th scope="col" className="px-6 py-3 text-left text-xs font-medium text-slate-300 uppercase tracking-wider">Size</th>
                            <th scope="col" className="px-6 py-3 text-left text-xs font-medium text-slate-300 uppercase tracking-wider">Pushed</th>
                            <th scope="col" className="relative px-6 py-3"><span className="sr-only">Copy Pull Command</span></th>
                        </tr>
                    </thead>
                    <tbody className="bg-slate-800 divide-y divide-slate-700">
                        {tags.map((tag) => (
                            <TagListItem 
                                key={tag.name} 
                                tag={tag} 
                                repositoryPath={repositoryPath}
                            />
                        ))}
                    </tbody>
                </table>
            </div>
        </div>
    );
};

interface TagListItemProps {
    tag: ImageTag;
    repositoryPath: string;
}

const TagListItem: React.FC<TagListItemProps> = ({ tag, repositoryPath }) => {
    const [copyStatus, setCopyStatus] = useState('Copy');
    const pullCommand = `docker pull ${repositoryPath}:${tag.name}`;

    const handleCopy = (e: React.MouseEvent) => {
        e.stopPropagation();
        navigator.clipboard.writeText(pullCommand).then(() => {
        setCopyStatus('Copied!');
        setTimeout(() => setCopyStatus('Copy'), 2000);
        }).catch(err => {
        console.error('Failed to copy text: ', err);
        setCopyStatus('Failed');
        setTimeout(() => setCopyStatus('Copy'), 2000);
        });
    };

    return (
        <tr className="hover:bg-slate-700/50 transition-colors duration-150">
            <td className="px-6 py-4 whitespace-nowrap text-sm font-medium text-slate-100">
                {/* Changed from a button to plain text to disable navigation */}
                {tag.name}
            </td>
            <td className="px-6 py-4 whitespace-nowrap text-sm text-slate-400 font-mono" title={tag.digest}>
                {tag.digest.substring(0, 19)}...
            </td>
            <td className="px-6 py-4 whitespace-nowrap text-sm text-slate-300">{tag.osArch}</td>
            <td className="px-6 py-4 whitespace-nowrap text-sm text-slate-300">{tag.size}</td>
            <td className="px-6 py-4 whitespace-nowrap text-sm text-slate-300">{tag.pushedAt}</td>
            <td className="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
                <button
                    onClick={handleCopy}
                    title="Copy pull command"
                    className="p-1.5 rounded-md text-slate-400 hover:bg-slate-700 hover:text-slate-100 transition-colors flex items-center"
                >
                    <ClipboardIcon className="w-4 h-4 mr-2" />
                    {copyStatus}
                </button>
            </td>
        </tr>
    );
};

export default RepositoryTags;