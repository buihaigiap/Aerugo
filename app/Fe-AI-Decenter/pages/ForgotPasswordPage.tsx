import React, { useState } from 'react';
import { Link } from 'react-router-dom';
import { AerugoIcon } from '../components/icons/DockerIcon';
import Input from '../components/Input';
import Button from '../components/Button';
import { forgotPassword } from '../services/api';

const ForgotPasswordPage: React.FC = () => {
    const [email, setEmail] = useState('');
    const [isLoading, setIsLoading] = useState(false);
    const [error, setError] = useState<string | null>(null);
    const [isSubmitted, setIsSubmitted] = useState(false);

    const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();
        setError(null);

        if (!email) {
            setError('Please enter your email address.');
            return;
        }

        setIsLoading(true);
        try {
            // This currently uses the placeholder API function
            await forgotPassword({ email });
            setIsSubmitted(true);
        } catch (err) {
            // In a real scenario, you might not want to reveal if an email exists or not
            // So a generic success message is often better regardless of the outcome.
            // For now, we'll assume the happy path and show the success state.
            console.error(err);
            setIsSubmitted(true); 
        } finally {
            setIsLoading(false);
        }
    };

    return (
        <div className="min-h-screen bg-gradient-to-b from-slate-900 via-indigo-950 to-slate-900 text-slate-100 font-sans flex items-center justify-center p-4">
            <div className="w-full max-w-md">
                <div className="text-center mb-8">
                    <Link to="/" className="inline-block">
                        <AerugoIcon className="w-12 h-12 text-indigo-400" />
                    </Link>
                </div>
                
                <div className="bg-slate-800/80 backdrop-blur-sm border border-slate-700 rounded-lg p-8 shadow-2xl shadow-slate-950/50">
                    {isSubmitted ? (
                        <div className="text-center">
                            <h2 className="text-2xl font-bold text-slate-50 mb-2">Check Your Email</h2>
                            <p className="text-slate-400">
                                If an account exists for <strong className="font-medium text-slate-200">{email}</strong>, you will receive an email with instructions on how to reset your password.
                            </p>
                             <div className="mt-6">
                                <Link to="/" className="font-semibold text-indigo-400 hover:text-indigo-300">
                                    &larr; Back to Sign In
                                </Link>
                            </div>
                        </div>
                    ) : (
                        <>
                            <h2 className="text-2xl font-bold text-center text-slate-50 mb-2">
                                Forgot Password?
                            </h2>
                            <p className="text-center text-slate-400 mb-8">
                                Enter your email and we'll send you a reset link.
                            </p>
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
                                    Send Reset Link
                                </Button>
                            </form>
                        </>
                    )}
                </div>
                 {!isSubmitted && (
                    <div className="text-center mt-6">
                        <Link to="/" className="font-semibold text-indigo-400 hover:text-indigo-300">
                            &larr; Back to Sign In
                        </Link>
                    </div>
                )}
            </div>
        </div>
    );
};

export default ForgotPasswordPage;
