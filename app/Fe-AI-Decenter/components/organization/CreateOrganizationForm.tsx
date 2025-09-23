import React, { useState } from "react";
import { createOrganization } from "../../services/api";
import { CreateOrganizationRequest } from "../../types";
import Input from "../Input";
import Button from "../Button";

interface CreateOrganizationFormProps {
  token: string;
  onSuccess: () => void;
  onCancel: () => void;
}

const CreateOrganizationForm: React.FC<CreateOrganizationFormProps> = ({
  token,
  onSuccess,
  onCancel,
}) => {
  const [formData, setFormData] = useState<CreateOrganizationRequest>({
    name: "",
    display_name: "",
    description: "",
    avatar_url: "",
    website_url: "",
  });
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value } = e.target;
    setFormData((prev) => ({ ...prev, [name]: value }));
  };

  const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    if (!formData.name || !formData.display_name) {
      setError("Organization name and display name are required.");
      return;
    }
    // Basic validation for org name (URL-friendly)
    if (!/^[a-z0-9]+(?:-[a-z0-9]+)*$/.test(formData.name)) {
      setError(
        "Name must be lowercase, alphanumeric, and can contain hyphens."
      );
      return;
    }

    setIsLoading(true);
    setError(null);
    try {
      const payload: CreateOrganizationRequest = {
        ...formData,
        avatar_url: formData.avatar_url || undefined,
        website_url: formData.website_url || undefined,
      };
      await createOrganization(payload, token);
      onSuccess();
    } catch (err) {
      setError("Failed to create organization. Please try again.");
      console.error(err);
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="bg-slate-800 border border-slate-700 rounded-lg p-6 shadow-lg">
      <h3 className="text-xl font-bold text-slate-50 mb-4">
        Create New Organization
      </h3>
      <form onSubmit={handleSubmit} className="space-y-4">
        <Input
          id="display_name"
          name="display_name"
          label="Display Name"
          value={formData.display_name}
          onChange={handleChange}
          placeholder="My Awesome Team"
          required
        />
        <Input
          id="name"
          name="name"
          label="Organization Name (URL)"
          value={formData.name}
          onChange={handleChange}
          placeholder="my-awesome-team"
          required
        />
        <p className="text-xs text-slate-400 -mt-2 ml-1">
          Lowercase, numbers, and hyphens only.
        </p>

        <Input
          id="description"
          name="description"
          label="Description (Optional)"
          value={formData.description || ""}
          onChange={handleChange}
          placeholder="A brief description of your organization."
        />
        <Input
          id="website_url"
          name="website_url"
          label="Website URL (Optional)"
          type="url"
          value={formData.website_url || ""}
          onChange={handleChange}
          placeholder="https://example.com"
        />
        <Input
          id="avatar_url"
          name="avatar_url"
          label="Avatar URL (Optional)"
          type="url"
          value={formData.avatar_url || ""}
          onChange={handleChange}
          placeholder="https://example.com/logo.png"
        />

        {error && <p className="text-sm text-red-500">{error}</p>}

        <div className="flex justify-end items-center space-x-4 pt-2">
          <Button
            type="button"
            onClick={onCancel}
            className="bg-transparent hover:bg-slate-700 text-slate-300"
          >
            Cancel
          </Button>
          <Button type="submit" isLoading={isLoading} fullWidth={false}>
            Create Organization
          </Button>
        </div>
      </form>
    </div>
  );
};

export default CreateOrganizationForm;
