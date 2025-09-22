import React, { useState } from 'react';
import { useNavigate, Link } from 'react-router-dom';
import Input from './Input';
import Button from './Button';
import { API_BASE_URL } from '../config';

interface LoginFormProps {
  onLoginSuccess: (token: string) => void;
}

const LoginForm: React.FC<LoginFormProps> = ({ onLoginSuccess }) => {
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [error, setError] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const navigate = useNavigate();

  const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    setError(null);

    if (!email || !password) {
      setError('Please fill in all fields.');
      return;
    }

    setIsLoading(true);

    try {
      const response = await fetch(`${API_BASE_URL}/api/v1/auth/login`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ email, password }),
      });

      if (response.ok) {
        const data = await response.json();
        onLoginSuccess(data.token);
        navigate('/repositories', { replace: true });
      } else if (response.status === 401) {
        setError('Invalid email or password.');
      } else {
        setError('An unexpected error occurred. Please try again.');
      }
    } catch (err) {
      setError('Failed to connect to the server. Please check your network.');
      console.error(err);
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <form onSubmit={handleSubmit} className="space-y-6">
      <Input
        id="login-email"
        type="email"
        label="Email Address"
        value={email}
        onChange={(e) => setEmail(e.target.value)}
        placeholder="you@example.com"
        disabled={isLoading}
        autoComplete="email"
      />
      <div>
        <Input
            id="login-password"
            type="password"
            label="Password"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
            placeholder="••••••••"
            disabled={isLoading}
            autoComplete="current-password"
        />
        <div className="text-right mt-2">
            <Link 
                to="/forgot-password" 
                className="text-sm font-medium text-indigo-400 hover:text-indigo-300 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 focus:ring-offset-slate-800 rounded"
            >
                Forgot your password?
            </Link>
        </div>
      </div>
      {error && <p className="text-sm text-red-500">{error}</p>}
      <Button type="submit" isLoading={isLoading}>
        Sign In
      </Button>
    </form>
  );
};

export default LoginForm;