import React, { useState } from "react";
import { Webhook } from "../../types";
import Button from "../Button";
import Input from "../Input";
import { PlusIcon } from "../icons/PlusIcon";
import { RssIcon } from "../icons/RssIcon";

const mockWebhooks: Webhook[] = [];

const RepositoryWebhooks: React.FC = () => {
  const [showAddForm, setShowAddForm] = useState(false);

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <h3 className="text-xl font-bold text-slate-50">Webhooks</h3>
        {!showAddForm && (
          <Button onClick={() => setShowAddForm(true)} fullWidth={false}>
            <PlusIcon className="w-5 h-5 -ml-1 mr-2" />
            Add Webhook
          </Button>
        )}
      </div>

      {showAddForm && <AddWebhookForm onCancel={() => setShowAddForm(false)} />}

      {mockWebhooks.length > 0 ? (
        <div className="border border-slate-700 rounded-lg">
          <ul className="divide-y divide-slate-700">
            {mockWebhooks.map((hook) => (
              <WebhookListItem key={hook.id} hook={hook} />
            ))}
          </ul>
        </div>
      ) : (
        <div className="text-center py-10 px-4 border-2 border-dashed border-slate-700 rounded-lg">
          <RssIcon className="mx-auto h-12 w-12 text-slate-500" />
          <h3 className="mt-2 text-lg font-medium text-slate-300">
            No Webhooks Configured
          </h3>
          <p className="text-slate-400 mt-1">
            Add a webhook to be notified of repository events. This feature is
            coming soon.
          </p>
        </div>
      )}
    </div>
  );
};

const AddWebhookForm: React.FC<{ onCancel: () => void }> = ({ onCancel }) => {
  return (
    <div className="bg-slate-800/50 border border-slate-700 rounded-lg p-6">
      <form className="space-y-4">
        <Input
          id="webhook-url"
          name="url"
          label="Payload URL"
          type="url"
          placeholder="https://example.com/webhook"
          required
        />
        <Input
          id="webhook-secret"
          name="secret"
          label="Secret (Optional)"
          type="password"
          placeholder="A secure, random string"
        />

        <div>
          <label className="block text-sm font-medium text-slate-300 mb-2">
            Events
          </label>
          <div className="space-y-2">
            <label className="flex items-center text-slate-200">
              <input
                type="checkbox"
                className="h-4 w-4 rounded bg-slate-600 border-slate-500 text-blue-500 focus:ring-blue-500"
                defaultChecked
              />
              <span className="ml-3">Push</span>
            </label>
            <label className="flex items-center text-slate-200">
              <input
                type="checkbox"
                className="h-4 w-4 rounded bg-slate-600 border-slate-500 text-blue-500 focus:ring-blue-500"
              />
              <span className="ml-3">Delete</span>
            </label>
          </div>
        </div>

        <div className="flex justify-end items-center space-x-4 pt-2">
          <Button
            type="button"
            onClick={onCancel}
            className="bg-transparent hover:bg-slate-700 text-slate-300"
          >
            Cancel
          </Button>
          <Button
            type="submit"
            fullWidth={false}
            onClick={(e) => {
              e.preventDefault();
              onCancel();
            }}
          >
            Add Webhook
          </Button>
        </div>
      </form>
    </div>
  );
};

const WebhookListItem: React.FC<{ hook: Webhook }> = ({ hook }) => {
  const isSuccess = hook.lastDelivery?.status === "success";
  return (
    <li className="p-4 flex items-center justify-between">
      <div>
        <p className="text-md font-mono font-semibold text-slate-100 break-all">
          {hook.url}
        </p>
        <div className="flex items-center space-x-2 mt-2">
          {hook.lastDelivery && (
            <span
              className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${
                isSuccess
                  ? "bg-green-900/50 text-green-300"
                  : "bg-red-900/50 text-red-300"
              }`}
            >
              {isSuccess ? "Success" : "Failed"}
            </span>
          )}
          <span className="text-xs text-slate-400">
            {hook.lastDelivery
              ? `Last delivery: ${hook.lastDelivery.timestamp}`
              : "Never delivered"}
          </span>
        </div>
      </div>
    </li>
  );
};

export default RepositoryWebhooks;
