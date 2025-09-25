import React from 'react';
import { Link, useNavigate } from 'react-router-dom';
import RegisterForm from '../components/RegisterForm';
import AnimatedParticleBackground from '../components/AnimatedParticleBackground';
import AuthCard from '../components/AuthCard';

const RegisterPage: React.FC = () => {
    const navigate = useNavigate();

    const handleRegisterSuccess = () => {
        // After showing success message in the form, navigate to login
        setTimeout(() => {
            navigate('/login');
        }, 1500); // Match delay in RegisterForm
    };

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
          title="Create Your Account"
          subtitle="Join now to start pushing images."
        >
          <RegisterForm onRegisterSuccess={handleRegisterSuccess} />
        </AuthCard>
        
        <div className="text-center mt-8">
          <p className="text-slate-400">
            Already have an account?{' '}
            <Link
              to="/login"
              className="font-semibold text-indigo-400 hover:text-indigo-300 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 focus:ring-offset-slate-900 rounded transition-colors duration-200"
            >
              Sign In
            </Link>
          </p>
        </div>
      </div>
    </div>
  );
};

export default RegisterPage;