import React from 'react';

interface InputProps extends React.InputHTMLAttributes<HTMLInputElement> {
  label: string;
  id: string;
}

const Input: React.FC<InputProps> = ({ label, id, ...props }) => {
  return (
    <div>
      <label htmlFor={id} className="block text-sm font-medium text-slate-300 mb-2">
        {label}
      </label>
      <input
        id={id}
        {...props}
        className="block w-full px-4 py-2 bg-slate-700 border border-slate-600 rounded-md text-slate-100 placeholder-slate-400 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 sm:text-sm disabled:opacity-50 disabled:cursor-not-allowed"
      />
    </div>
  );
};

export default Input;