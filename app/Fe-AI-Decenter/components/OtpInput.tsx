import React, { useRef, ChangeEvent, KeyboardEvent, ClipboardEvent, useEffect } from 'react';

interface OtpInputProps {
  length?: number;
  value: string;
  onChange: (value: string) => void;
  disabled?: boolean;
}

const OtpInput: React.FC<OtpInputProps> = ({ length = 6, value, onChange, disabled }) => {
  const inputsRef = useRef<(HTMLInputElement | null)[]>([]);

  // Auto-focus the first empty input on mount or when value changes
  useEffect(() => {
    const firstEmptyIndex = value.length;
    if (firstEmptyIndex < length) {
      inputsRef.current[firstEmptyIndex]?.focus();
    }
  }, [value, length]);


  const handleChange = (e: ChangeEvent<HTMLInputElement>, index: number) => {
    const targetValue = e.target.value.replace(/[^0-9]/g, ''); // Only allow digits
    const newOtp = value.split('');
    
    // Handle typing a digit
    if (targetValue) {
        newOtp[index] = targetValue.slice(-1); // Take only the last typed digit
        onChange(newOtp.join(''));
        // Focus next input if a digit was entered
        if (index < length - 1) {
            inputsRef.current[index + 1]?.focus();
        }
    }
  };

  const handleKeyDown = (e: KeyboardEvent<HTMLInputElement>, index: number) => {
    if (e.key === 'Backspace') {
      e.preventDefault();
      const newOtp = value.split('');
      
      if (newOtp[index]) {
        // If current input has a value, delete it
        newOtp[index] = '';
      } else if (index > 0) {
        // If current input is empty, delete previous and move focus
        newOtp[index - 1] = '';
        inputsRef.current[index - 1]?.focus();
      }
      onChange(newOtp.join(''));

    } else if (e.key === 'ArrowLeft' && index > 0) {
      e.preventDefault();
      inputsRef.current[index - 1]?.focus();
    } else if (e.key === 'ArrowRight' && index < length - 1) {
      e.preventDefault();
      inputsRef.current[index + 1]?.focus();
    }
  };

  const handlePaste = (e: ClipboardEvent<HTMLDivElement>) => {
    e.preventDefault();
    const pastedData = e.clipboardData.getData('text').replace(/[^0-9]/g, '');
    if (pastedData) {
      const newOtp = pastedData.slice(0, length);
      onChange(newOtp);
      // Focus the input after the pasted content, or the last input
      const nextFocusIndex = Math.min(newOtp.length, length - 1);
      inputsRef.current[nextFocusIndex]?.focus();
    }
  };

  return (
    <div>
      <label htmlFor="otp-input-0" className="block text-sm font-medium text-slate-300 mb-2">
        6-Digit Code
      </label>
      <div className="flex items-center justify-between gap-2 sm:gap-3" onPaste={handlePaste}>
        {Array.from({ length }, (_, index) => (
          <input
            key={index}
            id={`otp-input-${index}`}
            ref={(el) => { inputsRef.current[index] = el; }}
            type="text"
            inputMode="numeric"
            maxLength={1}
            value={value[index] || ''}
            onChange={(e) => handleChange(e, index)}
            onKeyDown={(e) => handleKeyDown(e, index)}
            onFocus={(e) => e.target.select()}
            disabled={disabled}
            autoComplete="one-time-code"
            className="block w-full h-12 sm:h-14 text-center text-xl sm:text-2xl font-semibold bg-slate-700 border border-slate-600 rounded-md text-slate-100 placeholder-slate-400 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-indigo-500 disabled:opacity-50 disabled:cursor-not-allowed transition-all duration-200 ease-in-out focus:shadow-[0_0_15px_rgba(99,102,241,0.5)]"
          />
        ))}
      </div>
    </div>
  );
};

export default OtpInput;
