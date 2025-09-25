

import React, { useState, useEffect } from 'react';
import { Link, useLocation, useNavigate } from 'react-router-dom';
import Input from '../components/Input';
import Button from '../components/Button';
import { verifyOtpAndResetPassword } from '../services/api';
import AnimatedParticleBackground from '../components/AnimatedParticleBackground';
import AuthCard from '../components/AuthCard';
import OtpInput from '../components/OtpInput';

const VerifyOtpPage: React.FC = () => {
    const location = useLocation();
    const navigate = useNavigate();
    
    // Email is passed from the previous page
    const email = location.state?.email;

    const [otpCode, setOtpCode] = useState('');
    const [newPassword, setNewPassword] = useState('');
    const [confirmPassword, setConfirmPassword] = useState('');
    const [isLoading, setIsLoading] = useState(false);
    const [error, setError] = useState<string | null>(null);
    const [isSuccess, setIsSuccess] = useState(false);

    useEffect(() => {
        if (!email) {
            // If the user lands here directly without an email, redirect them back
            navigate('/forgot-password');
        }
    }, [email, navigate]);

    const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();
        setError(null);

        if (!otpCode || !newPassword || !confirmPassword) {
            setError('Please fill in all fields.');
            return;
        }
        if (newPassword.length < 8) {
            setError('New password must be at least 8 characters long.');
            return;
        }
        if (newPassword !== confirmPassword) {
            setError('Passwords do not match.');
            return;
        }
        if (otpCode.length !== 6) {
            setError('Please enter a valid 6-digit OTP code.');
            return;
        }

        setIsLoading(true);
        try {
            await verifyOtpAndResetPassword({
                email,
                otp_code: otpCode,
                new_password: newPassword,
                confirm_password: confirmPassword,
            });
            setIsSuccess(true);
            setTimeout(() => {
                navigate('/login'); // Redirect to login on success
            }, 3000);
        } catch (err: any) {
            if (err.status === 400) {
                 setError("Invalid or expired OTP. Please try again.");
            } else {
                setError("An error occurred. Please try again later.");
            }
            console.error(err);
        } finally {
            setIsLoading(false);
        }
    };

    if (!email) {
        // Render nothing while redirecting
        return null;
    }

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
                     title={isSuccess ? "Success!" : "Check Your Email"}
                     subtitle={isSuccess 
                        ? "Your password has been reset. Redirecting you to sign in..." 
                        : `We've sent a 6-digit code to ${email}.`}
                >
                     {isSuccess ? (
                        <div className="text-center py-8">
                            {/* Success message is now in the subtitle */}
                        </div>
                    ) : (
                        <form onSubmit={handleSubmit} className="space-y-6">
                            <OtpInput
                                value={otpCode}
                                onChange={setOtpCode}
                                disabled={isLoading}
                            />
                            <Input
                                id="new-password"
                                type="password"
                                label="New Password"
                                value={newPassword}
                                onChange={(e) => setNewPassword(e.target.value)}
                                placeholder="••••••••"
                                disabled={isLoading}
                                autoComplete="new-password"
                            />
                            <Input
                                id="confirm-password"
                                type="password"
                                label="Confirm New Password"
                                value={confirmPassword}
                                onChange={(e) => setConfirmPassword(e.target.value)}
                                placeholder="••••••••"
                                disabled={isLoading}
                                autoComplete="new-password"
                            />
                            {error && <p className="text-sm text-red-500">{error}</p>}
                            <Button type="submit" isLoading={isLoading}>
                                Reset Password
                            </Button>
                        </form>
                    )}
                </AuthCard>
            </div>
        </div>
    );
};

export default VerifyOtpPage;