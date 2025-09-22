
import React, { useState, useRef } from 'react';
import { Link } from 'react-router-dom';
import LoginForm from '../components/LoginForm';
import RegisterForm from '../components/RegisterForm';
import { AuthMode } from '../types';
import { AerugoIcon } from '../components/icons/DockerIcon';
import { BriefcaseIcon } from '../components/icons/BriefcaseIcon';
import { ShieldCheckIcon } from '../components/icons/ShieldCheckIcon';
import { CodeBracketIcon } from '../components/icons/CodeBracketIcon';
import { ServerStackIcon } from '../components/icons/ServerStackIcon';
import { UsersIcon } from '../components/icons/UsersIcon';
import { CloudIcon } from '../components/icons/CloudIcon';
import { RocketLaunchIcon } from '../components/icons/RocketLaunchIcon';


interface AuthPageProps {
  onLoginSuccess: (token: string) => void;
}

const AuthPage: React.FC<AuthPageProps> = ({ onLoginSuccess }) => {
  const [authMode, setAuthMode] = useState<AuthMode>(AuthMode.Register);
  const authSectionRef = useRef<HTMLDivElement>(null);
  const introductionSectionRef = useRef<HTMLDivElement>(null);

  const switchMode = (mode: AuthMode) => {
    setAuthMode(mode);
  };
  
  const handleRegisterSuccess = () => {
    setAuthMode(AuthMode.Login);
  };

  const handleGetStartedClick = () => {
    setAuthMode(AuthMode.Register);
    authSectionRef.current?.scrollIntoView({ behavior: 'smooth' });
  };

  const handleDocsClick = () => {
    introductionSectionRef.current?.scrollIntoView({ behavior: 'smooth' });
  };

  return (
    <div className="min-h-screen bg-gradient-to-b from-slate-900 via-indigo-950 to-slate-900 text-slate-100 font-sans overflow-x-hidden">
      {/* Header */}
      <header className="py-4 px-4 sm:px-6 lg:px-8 bg-slate-900/50 backdrop-blur-sm sticky top-0 z-20 border-b border-slate-800/50">
        <nav className="flex items-center justify-between">
          <div className="flex items-center space-x-3">
            <AerugoIcon className="w-8 h-8 text-indigo-400" />
            <span className="text-xl font-bold text-slate-50">Aerugo Registry</span>
          </div>
          <div className="flex items-center space-x-6">
             <button
                onClick={handleDocsClick}
                className="font-semibold text-indigo-400 hover:text-indigo-300 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 focus:ring-offset-slate-900 rounded px-2 py-1 transition-colors"
             >
                Introduction
                           
              </button>
              <Link
                to="/docs"
                className="font-semibold text-indigo-400 hover:text-indigo-300 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 focus:ring-offset-slate-900 rounded px-2 py-1 transition-colors"
              >
                Docs
              </Link>
            <button
                onClick={() => {
                  setAuthMode(AuthMode.Login);
                  authSectionRef.current?.scrollIntoView({ behavior: 'smooth' });
                }}
                className="font-semibold text-indigo-400 hover:text-indigo-300 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 focus:ring-offset-slate-900 rounded px-2 py-1 transition-colors"
              >
                Sign In
              </button>
          </div>
        </nav>
      </header>

      {/* Hero Section */}
      <main className="pt-20 pb-16 sm:pt-24 lg:pt-32 text-center">
        <div className="max-w-4xl mx-auto px-4">
          <h1 className="text-4xl sm:text-5xl lg:text-6xl font-extrabold tracking-tight bg-gradient-to-r from-slate-100 to-indigo-300 text-transparent bg-clip-text">
            The Modern, Secure Container Registry
          </h1>
          <p className="mt-6 max-w-2xl mx-auto text-lg text-slate-400">
            Streamline your development workflow with a private, scalable, and easy-to-use registry for all your container images.
          </p>
          <div className="mt-8 flex justify-center gap-4">
            <button
              onClick={handleGetStartedClick}
              className="px-8 py-3 bg-indigo-600 hover:bg-indigo-500 text-white font-semibold rounded-md shadow-lg transition-transform transform hover:scale-105 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 focus:ring-offset-slate-900"
            >
              Get Started for Free
            </button>
          </div>
        </div>
      </main>

      {/* Introduction Section */}
      <section ref={introductionSectionRef} id="introduction" className="py-20 bg-transparent scroll-mt-20">
        <div className="max-w-5xl mx-auto px-4 sm:px-6 lg:px-8">
            <div className="text-center">
                <h2 className="text-3xl font-bold text-slate-50">Built for Performance, Security, and Scale</h2>
                <p className="mt-4 max-w-3xl mx-auto text-lg text-slate-400">
                    Aerugo is a next-generation, distributed, and multi-tenant container registry built with Rust. It is designed for high performance and scalability, leveraging an S3-compatible object storage backend.
                </p>
            </div>
            <div className="mt-16 grid gap-8 md:grid-cols-2 lg:grid-cols-4">
                <IntroFeature
                    icon={<ServerStackIcon className="h-8 w-8 text-indigo-400" />}
                    title="Distributed & Highly Available"
                    description="Designed to run in a clustered environment with no single point of failure."
                />
                 <IntroFeature
                    icon={<UsersIcon className="h-8 w-8 text-indigo-400" />}
                    title="Multi-tenancy"
                    description="First-class support for users and organizations with granular access control."
                />
                 <IntroFeature
                    icon={<CloudIcon className="h-8 w-8 text-indigo-400" />}
                    title="S3-Compatible Backend"
                    description="Uses any S3-compatible object storage for durability and infinite scalability."
                />
                 <IntroFeature
                    icon={<RocketLaunchIcon className="h-8 w-8 text-indigo-400" />}
                    title="Written in Rust"
                    description="Provides memory safety, concurrency, and performance for a secure, efficient core."
                />
            </div>
        </div>
      </section>

      {/* Features Section */}
      <section className="py-20 bg-slate-900/30">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="text-center">
            <h2 className="text-3xl font-bold text-slate-50">Built for Developers and Teams</h2>
            <p className="mt-4 text-lg text-slate-400">Everything you need, nothing you don't.</p>
          </div>
          <div className="mt-12 grid gap-8 md:grid-cols-3">
            <FeatureCard
              icon={<BriefcaseIcon className="h-8 w-8 text-indigo-400" />}
              title="Organize Repositories"
              description="Group your repositories under organizations to easily manage access and billing for your entire team."
            />
            <FeatureCard
              icon={<ShieldCheckIcon className="h-8 w-8 text-indigo-400" />}
              title="Secure & Private"
              description="Control who can see and pull your images with public/private repositories and fine-grained permissions."
            />
            <FeatureCard
              icon={<CodeBracketIcon className="h-8 w-8 text-indigo-400" />}
              title="Developer Friendly"
              description="A clean, intuitive UI and a straightforward API make managing your images a breeze. Works with standard Docker commands."
            />
          </div>
        </div>
      </section>

      {/* Auth Section */}
      <section ref={authSectionRef} className="py-20">
        <div className="w-full max-w-md mx-auto px-4">
            <h2 className="text-3xl font-bold text-center text-slate-50 mb-2">
                {authMode === AuthMode.Login ? 'Welcome Back' : 'Create Your Account'}
            </h2>
            <p className="text-center text-slate-400 mb-8">
            {authMode === AuthMode.Login ? 'Sign in to manage your repositories.' : 'Join now to start pushing images.'}
            </p>
            
            <div className="bg-slate-800/80 backdrop-blur-sm border border-slate-700 rounded-lg p-8 shadow-2xl shadow-slate-950/50">
            {authMode === AuthMode.Login ? (
                <LoginForm onLoginSuccess={onLoginSuccess} />
            ) : (
                <RegisterForm onRegisterSuccess={handleRegisterSuccess} />
            )}
            </div>
            
            <div className="text-center mt-6">
            {authMode === AuthMode.Login ? (
                <p className="text-slate-400">
                Don't have an account?{' '}
                <button
                    onClick={() => switchMode(AuthMode.Register)}
                    className="font-semibold text-indigo-400 hover:text-indigo-300 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 focus:ring-offset-slate-900 rounded"
                >
                    Sign Up
                </button>
                </p>
            ) : (
                <p className="text-slate-400">
                Already have an account?{' '}
                <button
                    onClick={() => switchMode(AuthMode.Login)}
                    className="font-semibold text-indigo-400 hover:text-indigo-300 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 focus:ring-offset-slate-900 rounded"
                >
                    Sign In
                </button>
                </p>
            )}
            </div>
        </div>
      </section>

      {/* Footer */}
      <footer className="bg-transparent py-6">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 text-center text-slate-500">
          <p>&copy; {new Date().getFullYear()} Aerugo Registry. All rights reserved.</p>
          <div className="mt-2">
             <Link to="/docs#tos" className="text-sm text-slate-400 hover:text-indigo-400 transition-colors">
                Terms of Service
             </Link>
          </div>
        </div>
      </footer>
    </div>
  );
};


interface IntroFeatureProps {
    icon: React.ReactNode;
    title: string;
    description: string;
}

const IntroFeature: React.FC<IntroFeatureProps> = ({ icon, title, description }) => (
    <div className="flex flex-col items-center text-center">
        <div className="flex items-center justify-center h-16 w-16 rounded-full bg-slate-800 mb-4 border border-slate-700">
            {icon}
        </div>
        <h3 className="text-lg font-semibold text-slate-100">{title}</h3>
        <p className="mt-1 text-slate-400">{description}</p>
    </div>
);


interface FeatureCardProps {
    icon: React.ReactNode;
    title: string;
    description: string;
}

const FeatureCard: React.FC<FeatureCardProps> = ({ icon, title, description }) => (
    <div className="flex flex-col items-center text-center p-6 bg-slate-800/70 backdrop-blur-sm rounded-lg border border-slate-700/80">
        <div className="flex items-center justify-center h-16 w-16 rounded-full bg-slate-700/80 mb-4">
            {icon}
        </div>
        <h3 className="text-xl font-semibold text-slate-100">{title}</h3>
        <p className="mt-2 text-slate-400">{description}</p>
    </div>
);


export default AuthPage;