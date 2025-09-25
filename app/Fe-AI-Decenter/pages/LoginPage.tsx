import React from 'react';
import { Link } from 'react-router-dom';
import LoginForm from '../components/LoginForm';
import AnimatedParticleBackground from '../components/AnimatedParticleBackground';
import AuthCard from '../components/AuthCard';

interface LoginPageProps {
  onLoginSuccess: (token: string) => void;
}

const LoginPage: React.FC<LoginPageProps> = ({ onLoginSuccess }) => {
  return (
    <div className="min-h-screen bg-slate-900 text-slate-100 font-sans flex items-center justify-center p-4 relative overflow-hidden">
      <AnimatedParticleBackground />
      <div className="relative z-10 w-full max-w-md">
        <div className="text-center mb-8">
          <Link to="/" className="inline-block transition-transform duration-300 hover:scale-110">
            <img src="/components/icons/logo.png" alt="Aerugo Logo" className="h-[100px] w-[100px]" />
          </Link>
        </div>

        <AuthCard
          title="Welcome Back"
          subtitle="Sign in to manage your repositories."
        >
          <LoginForm onLoginSuccess={onLoginSuccess} />
        </AuthCard>

        <div className="text-center mt-8">
          <p className="text-slate-400">
            Don't have an account?{' '}
            <Link
              to="/register"
              className="font-semibold text-indigo-400 hover:text-indigo-300 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 focus:ring-offset-slate-900 rounded transition-colors duration-200"
            >
              Sign Up
            </Link>
          </p>
        </div>
      </div>
    </div>
  );
};

export default LoginPage;