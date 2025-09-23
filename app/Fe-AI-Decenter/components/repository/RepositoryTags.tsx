import React, { useState } from "react";
import { ImageTag } from "../../types";
import { ClipboardIcon } from "../icons/ClipboardIcon";
import { TagIcon } from "../icons/TagIcon";
import RepositoryTagDetail from "./RepositoryTagDetail";

const mockTags: ImageTag[] = [
  {
    name: "latest",
    digest:
      "sha256:a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2",
    osArch: "linux/amd64",
    size: "128 MB",
    pushedAt: "3 hours ago",
    config: {
      created: "2024-07-28T10:30:00Z",
      dockerVersion: "20.10.7",
      osArch: "linux/amd64",
      labels: {
        maintainer: "Aerugo Team <contact@aerugo.io>",
        version: "1.2.1",
      },
    },
    history: [
      { command: "CMD", details: '["/bin/sh", "-c", "node server.js"]' },
      { command: "EXPOSE", details: "8080" },
      { command: "COPY", details: ". ." },
      { command: "RUN", details: "npm install --production" },
      { command: "WORKDIR", details: "/app" },
      { command: "FROM", details: "node:18-alpine" },
    ],
    layers: [
      { digest: "sha256:1a2b3c4d...", size: "5.5 MB" },
      { digest: "sha256:5e6f7g8h...", size: "12.1 MB" },
      { digest: "sha256:9i0j1k2l...", size: "110.4 MB" },
    ],
  },
  {
    name: "v1.2.1",
    digest:
      "sha256:f0e9d8c7b6a5f0e9d8c7b6a5f0e9d8c7b6a5f0e9d8c7b6a5f0e9d8c7b6a5f0e9",
    osArch: "linux/amd64",
    size: "127 MB",
    pushedAt: "2 days ago",
    config: {
      created: "2024-07-26T14:00:00Z",
      dockerVersion: "20.10.7",
      osArch: "linux/amd64",
      labels: {
        maintainer: "Aerugo Team <contact@aerugo.io>",
        version: "1.2.1",
      },
    },
    history: [
      { command: "CMD", details: '["/bin/sh", "-c", "node server.js"]' },
      { command: "EXPOSE", details: "8080" },
      { command: "COPY", details: ". ." },
      { command: "RUN", details: "npm install --production" },
      { command: "WORKDIR", details: "/app" },
      { command: "FROM", details: "node:18-alpine" },
    ],
    layers: [
      { digest: "sha256:1a2b3c4d...", size: "5.5 MB" },
      { digest: "sha256:5e6f7g8h...", size: "12.1 MB" },
      { digest: "sha256:9i0j1k2l...", size: "109.4 MB" },
    ],
  },
  {
    name: "v1.2.0",
    digest:
      "sha256:b1a2c3d4e5f6b1a2c3d4e5f6b1a2c3d4e5f6b1a2c3d4e5f6b1a2c3d4e5f6b1a2",
    osArch: "linux/amd64",
    size: "126 MB",
    pushedAt: "1 week ago",
    config: {
      created: "2024-07-21T09:15:00Z",
      dockerVersion: "20.10.6",
      osArch: "linux/amd64",
      labels: {
        maintainer: "Aerugo Team <contact@aerugo.io>",
        version: "1.2.0",
      },
    },
    history: [
      { command: "CMD", details: '["/bin/sh", "-c", "node server.js"]' },
      { command: "EXPOSE", details: "8080" },
      { command: "COPY", details: ". ." },
      { command: "RUN", details: "npm install" },
      { command: "WORKDIR", details: "/app" },
      { command: "FROM", details: "node:18-alpine" },
    ],
    layers: [
      { digest: "sha256:1a2b3c4d...", size: "5.5 MB" },
      { digest: "sha256:5e6f7g8h...", size: "12.1 MB" },
      { digest: "sha256:3m4n5o6p...", size: "108.4 MB" },
    ],
  },
];

interface RepositoryTagsProps {
  repositoryPath: string;
}

const RepositoryTags: React.FC<RepositoryTagsProps> = ({ repositoryPath }) => {
  const [selectedTag, setSelectedTag] = useState<ImageTag | null>(null);

  if (selectedTag) {
    return (
      <RepositoryTagDetail
        tag={selectedTag}
        repositoryPath={repositoryPath}
        onBack={() => setSelectedTag(null)}
      />
    );
  }

  if (mockTags.length === 0) {
    return (
      <div className="text-center py-10 px-4 border-2 border-dashed border-slate-700 rounded-lg">
        <TagIcon className="mx-auto h-12 w-12 text-slate-500" />
        <h3 className="mt-2 text-lg font-medium text-slate-300">
          No Tags Found
        </h3>
        <p className="text-slate-400 mt-1">
          This repository is empty. Push an image to see its tags here.
        </p>
      </div>
    );
  }

  return (
    <div className="overflow-x-auto animate-fade-in-up">
      <div className="border border-slate-700 rounded-lg">
        <table className="min-w-full divide-y divide-slate-700">
          <thead className="bg-slate-800/50">
            <tr>
              <th
                scope="col"
                className="px-6 py-3 text-left text-xs font-medium text-slate-300 uppercase tracking-wider"
              >
                Tag
              </th>
              <th
                scope="col"
                className="px-6 py-3 text-left text-xs font-medium text-slate-300 uppercase tracking-wider"
              >
                Digest
              </th>
              <th
                scope="col"
                className="px-6 py-3 text-left text-xs font-medium text-slate-300 uppercase tracking-wider"
              >
                OS/Arch
              </th>
              <th
                scope="col"
                className="px-6 py-3 text-left text-xs font-medium text-slate-300 uppercase tracking-wider"
              >
                Size
              </th>
              <th
                scope="col"
                className="px-6 py-3 text-left text-xs font-medium text-slate-300 uppercase tracking-wider"
              >
                Pushed
              </th>
              <th scope="col" className="relative px-6 py-3">
                <span className="sr-only">Copy Pull Command</span>
              </th>
            </tr>
          </thead>
          <tbody className="bg-slate-800 divide-y divide-slate-700">
            {mockTags.map((tag) => (
              <TagListItem
                key={tag.name}
                tag={tag}
                repositoryPath={repositoryPath}
                onSelectTag={() => setSelectedTag(tag)}
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
  onSelectTag: () => void;
}

const TagListItem: React.FC<TagListItemProps> = ({
  tag,
  repositoryPath,
  onSelectTag,
}) => {
  const [copyStatus, setCopyStatus] = useState("Copy");
  const pullCommand = `docker pull ${repositoryPath}:${tag.name}`;

  const handleCopy = (e: React.MouseEvent) => {
    e.stopPropagation(); 
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
    <tr className="hover:bg-slate-700/50 transition-colors duration-150">
      <td className="px-6 py-4 whitespace-nowrap text-sm font-medium">
        <button
          onClick={onSelectTag}
          className="text-indigo-400 hover:underline focus:outline-none"
        >
          {tag.name}
        </button>
      </td>
      <td
        className="px-6 py-4 whitespace-nowrap text-sm text-slate-400 font-mono"
        title={tag.digest}
      >
        {tag.digest.substring(0, 19)}...
      </td>
      <td className="px-6 py-4 whitespace-nowrap text-sm text-slate-300">
        {tag.osArch}
      </td>
      <td className="px-6 py-4 whitespace-nowrap text-sm text-slate-300">
        {tag.size}
      </td>
      <td className="px-6 py-4 whitespace-nowrap text-sm text-slate-300">
        {tag.pushedAt}
      </td>
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
