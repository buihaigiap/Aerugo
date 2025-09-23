import React, { useState, useEffect } from "react";
import { Link, useLocation, useNavigate } from "react-router-dom";
import { AerugoIcon } from "../components/icons/DockerIcon";
import Input from "../components/Input";
import Button from "../components/Button";
import { VerifyOtpAndResetPassword } from "../services/api";

const VerifyOtpPage: React.FC = () => {
  const location = useLocation();
  const navigate = useNavigate();

  const email = location.state?.email || "";
  const [otpCode, setOtpCode] = useState("");
  const [newPassword, setNewPassword] = useState("");
  const [confirmPassword, setConfirmPassword] = useState("");
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [isSuccess, setIsSuccess] = useState(false);

  useEffect(() => {
    if (!email) {
      navigate("/forgot-password");
    }
  }, [email, navigate]);

  const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    setError(null);

    if (!otpCode || !newPassword || !confirmPassword) {
      setError("Please fill in all fields.");
      return;
    }
    if (newPassword.length < 8) {
      setError("Password must be at least 8 characters long.");
      return;
    }
    if (newPassword !== confirmPassword) {
      setError("Passwords do not match.");
      return;
    }

    setIsLoading(true);
    try {
      await VerifyOtpAndResetPassword({
        email,
        otp_code: otpCode,
        new_password: newPassword,
        confirm_password: confirmPassword,
      });
      setIsSuccess(true);
      setTimeout(() => {
        navigate("/");
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
    return null;
  }

  return (
    <div className="min-h-screen bg-gradient-to-b from-slate-900 via-indigo-950 to-slate-900 text-slate-100 font-sans flex items-center justify-center p-4">
      <div className="w-full max-w-md">
        <div className="text-center mb-8">
          <Link to="/" className="inline-block">
            <AerugoIcon className="w-12 h-12 text-indigo-400" />
          </Link>
        </div>

        <div className="bg-slate-800/80 backdrop-blur-sm border border-slate-700 rounded-lg p-8 shadow-2xl shadow-slate-950/50">
          {isSuccess ? (
            <div className="text-center">
              <h2 className="text-2xl font-bold text-slate-50 mb-2">
                Password Reset Successfully!
              </h2>
              <p className="text-slate-400">
                You can now sign in with your new password. Redirecting you to
                the sign in page...
              </p>
            </div>
          ) : (
            <>
              <h2 className="text-2xl font-bold text-center text-slate-50 mb-2">
                Check Your Email
              </h2>
              <p className="text-center text-slate-400 mb-8">
                We've sent a 6-digit code to{" "}
                <strong className="font-medium text-slate-200">{email}</strong>.
                Enter it below to reset your password.
              </p>
              <form onSubmit={handleSubmit} className="space-y-6">
                <Input
                  id="otp-code"
                  type="text"
                  label="6-Digit Code"
                  value={otpCode}
                  onChange={(e) => setOtpCode(e.target.value)}
                  placeholder="123456"
                  disabled={isLoading}
                  autoComplete="one-time-code"
                  maxLength={6}
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
              <div className="text-center mt-6">
                <Link
                  to="/"
                  className="font-semibold text-indigo-400 hover:text-indigo-300"
                >
                  &larr; Back to Forgot Password
                </Link>
              </div>
            </>
          )}
        </div>
      </div>
    </div>
  );
};

export default VerifyOtpPage;
