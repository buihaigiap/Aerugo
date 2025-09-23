import React from "react";
import { Organization } from "../../types";
import { AerugoIcon } from "../icons/DockerIcon";

interface OrganizationListItemProps {
  organization: Organization;
  isSelected: boolean;
  onSelect: () => void;
}

const OrganizationListItem: React.FC<OrganizationListItemProps> = ({
  organization,
  isSelected,
  onSelect,
}) => {
  const baseClasses =
    "w-full text-left p-4 sm:p-6 transition-all duration-200 easy-in-out focus:outline-none tranform";
  const selectedClasses = "bg-indigo-900/50 scale-[1.02]";
  const hoverClasses = "hover:bg-slate-700/50 hover:scale-[1.02]";

  return (
    <li>
      <button
        onClick={onSelect}
        className={`${baseClasses} ${
          isSelected ? selectedClasses : hoverClasses
        }`}
      >
        <div className="flex items-center">
          <div className="flex-shrink-0">
            {organization.avatar_url ? (
              <img
                className="h-12 w-12 rounded-full object-cover"
                src={organization.avatar_url}
                alt={`${organization.display_name} avatar`}
              />
            ) : (
              <span className="h-12 w-12 rounded-full bg-slate-700 flex items-center justify-center">
                <AerugoIcon className="h-7 w-7 text-slate-400" />
              </span>
            )}
          </div>
          <div className="ml-4 flex-1 text-left">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-lg font-semibold text-slate-50">
                  {organization.display_name}
                </p>
                <p className="text-sm text-slate-400">@{organization.name}</p>
              </div>
            </div>
            {organization.description && (
              <p className="mt-2 text-sm text-slate-300">
                {organization.description}
              </p>
            )}
          </div>
        </div>
      </button>
    </li>
  );
};

export default OrganizationListItem;
