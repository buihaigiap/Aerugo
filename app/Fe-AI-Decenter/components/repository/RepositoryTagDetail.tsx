import React, { useState, useEffect } from "react";
import { fetchManifestByReference } from "../../services/api";
import { ManifestV2, ManifestListV2 } from "../../types";
import { ArrowLeftIcon } from "../icons/ArrowLeftIcon";
import { ClipboardIcon } from "../icons/ClipboardIcon";

interface RepositoryTagDetailProps {
  token: string;
  organizationName: string;
  repositoryName: string;
  tagName: string;
  onBack: () => void;
}

const formatBytes = (bytes: number, decimals = 2): string => {
  if (bytes === 0) return "0 Bytes";
  const k = 1024;
  const dm = decimals < 0 ? 0 : decimals;
  const sizes = ["Bytes", "KB", "MB", "GB", "TB", "PB"];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(dm)) + " " + sizes[i];
};

const CopyButton: React.FC<{ text: string }> = ({ text }) => {
  const [copyStatus, setCopyStatus] = useState("Copy");
  const handleCopy = () => {
    navigator.clipboard.writeText(text).then(() => {
      setCopyStatus("Copied!");
      setTimeout(() => setCopyStatus("Copy"), 2000);
    });
  };
  return (
    <button
      onClick={handleCopy}
      title={`Copy: ${text}`}
      className="p-1 rounded-md text-slate-400 hover:bg-slate-700 hover:text-slate-100 transition-colors"
    >
      <ClipboardIcon className="w-4 h-4" />
    </button>
  );
};

const RepositoryTagDetail: React.FC<RepositoryTagDetailProps> = ({
  token,
  organizationName,
  repositoryName,
  tagName,
  onBack,
}) => {
  const [manifest, setManifest] = useState<ManifestV2 | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const getDetails = async () => {
      setIsLoading(true);
      setError(null);
      try {
        // First, fetch the manifest for the tag
        const manifestData = await fetchManifestByReference(
          organizationName,
          repositoryName,
          tagName,
          token
        );

        if (
          manifestData.mediaType ===
          "application/vnd.docker.distribution.manifest.list.v2+json"
        ) {
          // It's a manifest list, find a suitable manifest (e.g., amd64) and fetch it
          const manifestList = manifestData as ManifestListV2;
          // Prefer amd64/linux, but fall back to the first one
          let targetManifest = manifestList.manifests.find(
            (m) =>
              m.platform.architecture === "amd64" && m.platform.os === "linux"
          );
          if (!targetManifest && manifestList.manifests.length > 0) {
            targetManifest = manifestList.manifests[0];
          }

          if (targetManifest) {
            // Fetch the actual image manifest using its digest
            const imageManifest = await fetchManifestByReference(
              organizationName,
              repositoryName,
              targetManifest.digest,
              token
            );
            if (
              imageManifest.mediaType ===
              "application/vnd.docker.distribution.manifest.v2+json"
            ) {
              setManifest(imageManifest as ManifestV2);
            } else {
              throw new Error(
                `Resolved manifest for digest ${targetManifest.digest} was not a V2 image manifest.`
              );
            }
          } else {
            throw new Error(
              "Manifest list is empty and contains no platforms to display."
            );
          }
        } else if (
          manifestData.mediaType ===
          "application/vnd.docker.distribution.manifest.v2+json"
        ) {
          // It's a regular manifest
          setManifest(manifestData as ManifestV2);
        } else {
          // This case handles potential future or unknown manifest types gracefully.
          throw new Error(
            `Unsupported manifest media type: ${
              (manifestData as any).mediaType
            }`
          );
        }
      } catch (err: any) {
        setError(
          err.message ||
            "Failed to load tag details. The manifest may not be available or supported."
        );
        console.error(err);
      } finally {
        setIsLoading(false);
      }
    };
    getDetails();
  }, [token, organizationName, repositoryName, tagName]);

  if (isLoading) {
    return (
      <div className="text-center py-10 text-slate-400">
        Loading tag details...
      </div>
    );
  }

  if (error) {
    return (
      <div className="text-center py-10 px-4 border-2 border-dashed border-red-700/50 bg-red-900/20 rounded-lg">
        <h3 className="text-lg font-medium text-red-300">
          Error Loading Tag Details
        </h3>
        <p className="text-red-400 mt-1">{error}</p>
        <button
          onClick={onBack}
          className="mt-4 flex items-center mx-auto text-sm text-indigo-400 hover:text-indigo-300 focus:outline-none focus:ring-2 focus:ring-indigo-500 rounded-md p-1"
        >
          <ArrowLeftIcon className="w-4 h-4 mr-2" />
          Back to tags
        </button>
      </div>
    );
  }

  if (!manifest) {
    return null;
  }

  const totalSize =
    manifest.layers.reduce((sum, layer) => sum + layer.size, 0) +
    manifest.config.size;

  return (
    <div className="space-y-6 animate-fade-in-up">
      <header>
        <button
          onClick={onBack}
          className="flex items-center text-sm text-indigo-400 hover:text-indigo-300 mb-2 focus:outline-none focus:ring-2 focus:ring-indigo-500 rounded-md p-1 -ml-1"
        >
          <ArrowLeftIcon className="w-4 h-4 mr-2" />
          Back to all tags
        </button>
        <h3 className="text-2xl font-bold text-slate-50">
          Tag: <span className="font-mono text-indigo-400">{tagName}</span>
        </h3>
      </header>

      <div className="grid grid-cols-1 sm:grid-cols-3 gap-4 text-center">
        <div className="bg-slate-800/50 p-3 rounded-lg border border-slate-700">
          <h4 className="text-xs font-medium text-slate-400 uppercase">
            Total Size
          </h4>
          <p className="text-lg font-bold text-slate-50 mt-1">
            {formatBytes(totalSize)}
          </p>
        </div>
        <div className="bg-slate-800/50 p-3 rounded-lg border border-slate-700">
          <h4 className="text-xs font-medium text-slate-400 uppercase">
            Layers
          </h4>
          <p className="text-lg font-bold text-slate-50 mt-1">
            {manifest.layers.length}
          </p>
        </div>
        <div className="bg-slate-800/50 p-3 rounded-lg border border-slate-700">
          <h4 className="text-xs font-medium text-slate-400 uppercase">
            Schema Version
          </h4>
          <p className="text-lg font-bold text-slate-50 mt-1">
            {manifest.schemaVersion}
          </p>
        </div>
      </div>

      <div>
        <h4 className="text-lg font-semibold text-slate-200 mb-3">
          Image Layers
        </h4>
        <div className="overflow-x-auto">
          <div className="border border-slate-700 rounded-lg">
            <table className="min-w-full divide-y divide-slate-700">
              <thead className="bg-slate-800/50">
                <tr>
                  <th
                    scope="col"
                    className="px-4 py-3 text-left text-xs font-medium text-slate-300 uppercase tracking-wider"
                  >
                    Digest
                  </th>
                  <th
                    scope="col"
                    className="px-4 py-3 text-left text-xs font-medium text-slate-300 uppercase tracking-wider"
                  >
                    Media Type
                  </th>
                  <th
                    scope="col"
                    className="px-4 py-3 text-right text-xs font-medium text-slate-300 uppercase tracking-wider"
                  >
                    Size
                  </th>
                </tr>
              </thead>
              <tbody className="bg-slate-800 divide-y divide-slate-700">
                <tr className="hover:bg-slate-700/50 transition-colors">
                  <td className="px-4 py-4 whitespace-nowrap text-sm font-mono text-slate-400">
                    <div className="flex items-center gap-2">
                      <span className="truncate" title={manifest.config.digest}>
                        {manifest.config.digest.substring(0, 20)}...
                      </span>
                      <CopyButton text={manifest.config.digest} />
                    </div>
                  </td>
                  <td className="px-4 py-4 whitespace-nowrap text-sm text-slate-300">
                    {manifest.config.mediaType} (config)
                  </td>
                  <td className="px-4 py-4 whitespace-nowrap text-right text-sm text-slate-300">
                    {formatBytes(manifest.config.size)}
                  </td>
                </tr>
                {manifest.layers.map((layer) => (
                  <tr
                    key={layer.digest}
                    className="hover:bg-slate-700/50 transition-colors"
                  >
                    <td className="px-4 py-4 whitespace-nowrap text-sm font-mono text-slate-400">
                      <div className="flex items-center gap-2">
                        <span className="truncate" title={layer.digest}>
                          {layer.digest.substring(0, 20)}...
                        </span>
                        <CopyButton text={layer.digest} />
                      </div>
                    </td>
                    <td className="px-4 py-4 whitespace-nowrap text-sm text-slate-300">
                      {layer.mediaType}
                    </td>
                    <td className="px-4 py-4 whitespace-nowrap text-right text-sm text-slate-300">
                      {formatBytes(layer.size)}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>
      </div>
    </div>
  );
};

export default RepositoryTagDetail;
