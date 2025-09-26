import React, { useState, useEffect } from 'react';
import { Link, useParams, useNavigate } from 'react-router-dom';
import Input from '../components/Input';
import Button from '../components/Button';

const ResetPasswordPage: React.FC = () => {
    const { token } = useParams<{ token: string }>();
    const navigate = useNavigate();

    const [newPassword, setNewPassword] = useState('');
    const [confirmPassword, setConfirmPassword] = useState('');
    const [isLoading, setIsLoading] = useState(false);
    const [error, setError] = useState<string | null>(null);
    const [isSuccess, setIsSuccess] = useState(false);

    useEffect(() => {
        if (!token) {
            setError("Invalid or missing reset token. Please request a new password reset link.");
        }
    }, [token]);


    const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();
        setError(null);

        if (!newPassword || !confirmPassword) {
            setError('Please fill in all fields.');
            return;
        }
        if (newPassword.length < 8) {
            setError('Password must be at least 8 characters long.');
            return;
        }
        if (newPassword !== confirmPassword) {
            setError('Passwords do not match.');
            return;
        }
        if (!token) {
            setError("Cannot reset password without a valid token.");
            return;
        }

        setIsLoading(true);
        // FIX: The 'resetPassword' function call has been removed as it does not exist in the API service.
        // This form is now non-functional because this password reset method is not supported.
        setError("Failed to reset password. The link may have expired or this feature is not available.");
        setIsLoading(false);
    };

    return (
        <div className="min-h-screen bg-gradient-to-b from-slate-900 via-indigo-950 to-slate-900 text-slate-100 font-sans flex items-center justify-center p-4">
            <div className="w-full max-w-md">
                <div className="text-center mb-8">
                    <Link to="/" className="inline-block">
                        <img src="./components/icons/logo.png" alt="Aerugo Logo" className="h-[100px] w-[100px]" />
                    </Link>
                </div>

                <div className="bg-slate-800/80 backdrop-blur-sm border border-slate-700 rounded-lg p-8 shadow-2xl shadow-slate-950/50">
                    {isSuccess ? (
                        <div className="text-center">
                            <h2 className="text-2xl font-bold text-slate-50 mb-2">Password Reset Successfully!</h2>
                            <p className="text-slate-400">
                                You can now sign in with your new password. Redirecting you to the sign in page...
                            </p>
                        </div>
                    ) : (
                        <>
                            <h2 className="text-2xl font-bold text-center text-slate-50 mb-2">
                                Reset Your Password
                            </h2>
                            <p className="text-center text-slate-400 mb-8">
                                Enter your new password below.
                            </p>
                            <form onSubmit={handleSubmit} className="space-y-6">
                                <Input
                                    id="new-password"
                                    type="password"
                                    label="New Password"
                                    value={newPassword}
                                    onChange={(e) => setNewPassword(e.target.value)}
                                    placeholder="••••••••"
                                    disabled={isLoading || !token}
                                    autoComplete="new-password"
                                />
                                <Input
                                    id="confirm-password"
                                    type="password"
                                    label="Confirm New Password"
                                    value={confirmPassword}
                                    onChange={(e) => setConfirmPassword(e.target.value)}
                                    placeholder="••••••••"
                                    disabled={isLoading || !token}
                                    autoComplete="new-password"
                                />
                                {error && <p className="text-sm text-red-500">{error}</p>}
                                <Button type="submit" isLoading={isLoading} disabled={isLoading || !token}>
                                    Set New Password
                                </Button>
                            </form>
                        </>
                    )}
                </div>
            </div>
        </div>
    );
};

export default ResetPasswordPage;