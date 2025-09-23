import React, { useState } from "react";
import { Repository } from "../../types";
import { LockIcon } from "../icons/LockIcon";
import { GlobeIcon } from "../icons/GlobeIcon";
import { ClipboardIcon } from "../icons/ClipboardIcon";

interface RepositoryListItemProps {
  repository: Repository;
  organizationName?: string;
  onSelect: (repository: Repository) => void;
}

const RepositoryListItem: React.FC<RepositoryListItemProps> = ({
  repository,
  organizationName,
  onSelect,
}) => {
  const isPrivate = !repository.is_public;
  const [copyStatus, setCopyStatus] = useState("Copy");

  const orgName = repository.organization?.name || organizationName;
  const pullCommand = `docker pull registry.example.com/${orgName}/${repository.name}:latest`;

  const handleCopy = () => {
    navigator.clipboard
      .writeText(pullCommand)
      .then(() => {
        setCopyStatus("Copied!");
        setTimeout(() => setCopyStatus("Copy"), 2000);
      })
      .catch((err) => {
        console.error("Failed to copy text: ", err);
        setCopyStatus("Failed");
        setTimeout(() => setCopyStatus("Copy"), 2000);
      });
  };

  return (
    <div className="bg-slate-800/50 hover:bg-slate-700/50 transition-colors duration-200 border border-slate-700 rounded-lg p-4 flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4">
      <div className="flex-1">
        <div className="flex items-center space-x-3">
          <button
            onClick={() => onSelect(repository)}
            className="text-lg font-semibold text-blue-400 hover:underline text-left"
          >
            <span className="text-slate-400">{orgName} /</span>{" "}
            {repository.name}
          </button>
          <span
            className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${
              isPrivate
                ? "bg-yellow-900/50 text-yellow-300"
                : "bg-green-900/50 text-green-300"
            }`}
          >
            {isPrivate ? (
              <LockIcon className="w-3 h-3 mr-1.5" />
            ) : (
              <GlobeIcon className="w-3 h-3 mr-1.5" />
            )}
            {isPrivate ? "private" : "public"}
          </span>
        </div>
        <p className="mt-1 text-sm text-slate-400">
          {repository.description || "No description provided."}
        </p>
      </div>
      <div className="w-full sm:w-auto">
        <div className="relative flex items-center w-full">
          <input
            type="text"
            readOnly
            value={pullCommand}
            className="bg-slate-900/70 text-slate-300 text-sm rounded-md pl-3 pr-10 py-1.5 w-full font-mono focus:outline-none"
          />
          <button
            onClick={handleCopy}
            title="Copy pull command"
            className="absolute right-1 top-1/2 -translate-y-1/2 p-1 rounded-md text-slate-400 hover:bg-slate-700 hover:text-slate-100 transition-colors"
          >
            <ClipboardIcon className="w-4 h-4" />
            <span className="sr-only">{copyStatus}</span>
          </button>
        </div>
      </div>
    </div>
  );
};

export default RepositoryListItem;
