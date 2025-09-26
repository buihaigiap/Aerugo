import React, { useState } from "react";
import { EyeIcon } from "./icons/EyeIcon";
import { EyeSlashIcon } from "./icons/EyeSlashIcon";

interface InputProps extends React.InputHTMLAttributes<HTMLInputElement> {
  label: string;
  id: string;
}

const Input: React.FC<InputProps> = ({ label, id, ...props }) => {
  const [showPassword, setShowPassword] = useState(false);
  const isPasswordField = props.type === "password";

  const toggleShowPassword = () => {
    setShowPassword((prev) => !prev);
  };

  const inputType = isPasswordField
    ? showPassword
      ? "text"
      : "password"
    : props.type;

  return (
    <div>
      <label
        htmlFor={id}
        className="block text-sm font-medium text-slate-300 mb-2"
      >
        {label}
      </label>
      <div className="relative">
        <input
          id={id}
          {...props}
          type={inputType}
          className={`block w-full px-4 py-2 bg-slate-700 border border-slate-600 rounded-md text-slate-100 placeholder-slate-400 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm disabled:opacity-50 disabled:cursor-not-allowed transition-all duration-200 ease-in-out focus:shadow-[0_0_15px_rgba(99,102,241,0.5)] ${
            isPasswordField ? "pr-10" : ""
          }`}
        />
        {isPasswordField && (
          <button
            type="button"
            onClick={toggleShowPassword}
            className="absolute inset-y-0 right-0 flex items-center pr-3 text-slate-400 hover:text-slate-200 focus:outline-none rounded-md"
            aria-label={showPassword ? "Hide password" : "Show password"}
          >
            {showPassword ? (
              <EyeSlashIcon className="h-5 w-5" />
            ) : (
              <EyeIcon className="h-5 w-5" />
            )}
          </button>
        )}
      </div>
    </div>
  );
};

export default Input;
