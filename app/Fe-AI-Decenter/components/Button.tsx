
import React from 'react';

interface ButtonProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  isLoading?: boolean;
  variant?: 'primary' | 'danger';
  fullWidth?: boolean;
}

const Button: React.FC<ButtonProps> = ({ 
    children, 
    isLoading = false, 
    variant = 'primary', 
    fullWidth = true,
    className,
    ...props 
}) => {
  const baseClasses = "flex justify-center items-center px-4 py-2.5 border border-transparent rounded-md font-semibold text-white focus:outline-none focus:ring-2 focus:ring-offset-2 disabled:opacity-75 disabled:cursor-not-allowed transition-all duration-200 ease-in-out transform active:scale-[0.98]";
  
  const variantClasses = {
    primary: "bg-gradient-to-r from-indigo-600 to-purple-600 hover:bg-gradient-to-l focus:ring-offset-slate-800 focus:ring-indigo-500",
    danger: "bg-red-600 hover:bg-red-700 focus:ring-offset-slate-800 focus:ring-red-500",
  };
  
  const widthClass = fullWidth ? "w-full" : "w-auto";

  const finalClassName = [
    baseClasses,
    variantClasses[variant],
    widthClass,
    className,
  ].filter(Boolean).join(' ');

  return (
    <button
      {...props}
      disabled={isLoading || props.disabled}
      className={finalClassName}
    >
      {isLoading ? (
        <svg
          className="animate-spin -ml-1 mr-3 h-5 w-5 text-white"
          xmlns="http://www.w3.org/2000/svg"
          fill="none"
          viewBox="0 0 24"
        >
          <circle
            className="opacity-25"
            cx="12"
            cy="12"
            r="10"
            stroke="currentColor"
            strokeWidth="4"
          ></circle>
          <path
            className="opacity-75"
            fill="currentColor"
            d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
          ></path>
        </svg>
      ) : null}
      {children}
    </button>
  );
};

export default Button;