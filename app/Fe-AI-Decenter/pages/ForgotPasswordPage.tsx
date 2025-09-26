import React, { useState } from 'react';
import { Link, useNavigate } from 'react-router-dom';
import Input from '../components/Input';
import Button from '../components/Button';
import { forgotPassword } from '../services/api';
import AnimatedParticleBackground from '../components/AnimatedParticleBackground';
import AuthCard from '../components/AuthCard';
import logoImage from '../components/icons/logo.png';

const ForgotPasswordPage: React.FC = () => {
    const [email, setEmail] = useState('');
    const [isLoading, setIsLoading] = useState(false);
    const [error, setError] = useState<string | null>(null);
    const navigate = useNavigate();

    const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();
        setError(null);

        if (!email) {
            setError('Please enter your email address.');
            return;
        }

        setIsLoading(true);
        try {
            await forgotPassword({ email });
            // On success, navigate to the OTP page with the email in state
            navigate('/verify-otp', { state: { email } });
        } catch (err: any) {
            if (err.status === 404) {
                setError("No account found with that email address.");
            } else {
                setError("An error occurred. Please try again later.");
            }
            console.error(err);
        } finally {
            setIsLoading(false);
        }
    };

    return (
        <div className="min-h-screen bg-slate-900 text-slate-100 font-sans flex items-center justify-center p-4 relative overflow-hidden">
            <AnimatedParticleBackground />
            <div className="relative z-10 w-full max-w-md">
                <div className="text-center mb-8">
                    <Link to="/" className="inline-block transition-transform duration-300 hover:scale-110">
                        <img src={logoImage} alt="Aerugo Logo" className="w-24 h-24" />
                    </Link>
                </div>

                <AuthCard
                    title="Forgot Password?"
                    subtitle="Enter your email and we'll send you a 6-digit code."
                >
                    <form onSubmit={handleSubmit} className="space-y-6">
                        <Input
                            id="forgot-email"
                            type="email"
                            label="Email Address"
                            value={email}
                            onChange={(e) => setEmail(e.target.value)}
                            placeholder="you@example.com"
                            disabled={isLoading}
                            autoComplete="email"
                        />
                        {error && <p className="text-sm text-red-500">{error}</p>}
                        <Button type="submit" isLoading={isLoading}>
                            Send Code
                        </Button>
                    </form>
                </AuthCard>

                <div className="text-center mt-8">
                    <Link to="/login" className="font-semibold text-indigo-400 hover:text-indigo-300 transition-colors duration-200">
                        &larr; Back to Sign In
                    </Link>
                </div>
            </div>
        </div>
    );
};

export default ForgotPasswordPage;