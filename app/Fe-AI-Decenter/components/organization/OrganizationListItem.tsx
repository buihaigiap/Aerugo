

import React from 'react';
import { Organization } from '../../types';

interface OrganizationListItemProps {
  organization: Organization;
  isSelected: boolean;
  onSelect: () => void;
}

const OrganizationListItem: React.FC<OrganizationListItemProps> = ({ organization, isSelected, onSelect }) => {
  
  return (
    <li>
        <button
            onClick={onSelect}
            className={`w-full text-left p-2.5 rounded-md border-l-4 transition-all duration-200 ease-in-out focus:outline-none focus-visible:ring-2 focus-visible:ring-indigo-500 focus-visible:ring-offset-2 focus-visible:ring-offset-slate-900
                ${isSelected 
                    ? 'border-indigo-500 bg-slate-700/50' 
                    : 'border-transparent hover:bg-slate-700/40'
                }`
            }
        >
            <div className="flex items-center space-x-3">
                <div className="flex-shrink-0">
                    <span className="h-8 w-8 rounded-full bg-slate-700 flex items-center justify-center border border-slate-600">
                        <img src="/components/icons/logo.png" alt="Organization Logo" className="h-[100px] w-[100px]" />
                    </span>
                </div>
                <div className="flex-1 text-left min-w-0">
                    <p className={`text-sm font-semibold truncate ${isSelected ? 'text-slate-50' : 'text-slate-200'}`}>{organization.display_name}</p>
                    <p className={`text-xs truncate ${isSelected ? 'text-indigo-300' : 'text-slate-400'}`}>@{organization.name}</p>
                </div>
            </div>
        </button>
    </li>
  );
};

export default OrganizationListItem;