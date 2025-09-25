

import React, { useState, useEffect } from 'react';
import { HashRouter, Routes, Route, Navigate } from 'react-router-dom';
import AuthPage from './pages/AuthPage';
import DocsPage from './pages/DocsPage';
import RepositoriesPage from './pages/RepositoriesPage';
import OrganizationsPage from './pages/OrganizationsPage';
import ProfilePage from './pages/ProfilePage';
import DashboardLayout from './components/layout/DashboardLayout';
import { User } from './types';
import { fetchCurrentUser } from './services/api';
import ForgotPasswordPage from './pages/ForgotPasswordPage';
import VerifyOtpPage from './pages/VerifyOtpPage';
import LoginPage from './pages/LoginPage';
import RegisterPage from './pages/RegisterPage';

const App: React.FC = () => {
  const [token, setToken] = useState<string | null>(localStorage.getItem('authToken'));
  const [currentUser, setCurrentUser] = useState<User | null>(null);
  const [isLoadingUser, setIsLoadingUser] = useState(true);

  useEffect(() => {
    const validateToken = async () => {
      const storedToken = localStorage.getItem('authToken');
      if (storedToken) {
        try {
          const user = await fetchCurrentUser(storedToken);
          setToken(storedToken);
          setCurrentUser(user);
        } catch (error) {
          // Invalid token, clear it
          localStorage.removeItem('authToken');
          setToken(null);
          setCurrentUser(null);
        } finally {
          setIsLoadingUser(false);
        }
      } else {
        setIsLoadingUser(false);
      }
    };
    validateToken();
  }, []);

  const handleLoginSuccess = async (newToken: string) => {
    localStorage.setItem('authToken', newToken);
    setToken(newToken);
    setIsLoadingUser(true);
    try {
      const user = await fetchCurrentUser(newToken);
      setCurrentUser(user);
    } catch (error) {
      console.error("Failed to fetch current user after login", error);
      handleLogout(); // Log out if user fetch fails
    } finally {
        setIsLoadingUser(false);
    }
  };

  const handleLogout = () => {
    localStorage.removeItem('authToken');
    setToken(null);
    setCurrentUser(null);
    // The navigation to the login page will be handled by the protected route logic.
  };

  if (isLoadingUser) {
    return (
      <div className="min-h-screen bg-slate-900 flex items-center justify-center">
        <p className="text-slate-400">Loading...</p>
      </div>
    );
  }

  return (
    <HashRouter>
      <Routes>
        <Route path="/" element={<AuthPage />} />
        
        {/* Unauthenticated Routes */}
        <Route
          path="/login"
          element={
            token && currentUser ? (
              <Navigate to="/repositories" replace />
            ) : (
              <LoginPage onLoginSuccess={handleLoginSuccess} />
            )
          }
        />
        <Route
          path="/register"
          element={
            token && currentUser ? (
              <Navigate to="/repositories" replace />
            ) : (
              <RegisterPage />
            )
          }
        />
        <Route path="/forgot-password" element={<ForgotPasswordPage />} />
        <Route path="/verify-otp" element={<VerifyOtpPage />} />

        {/* Public Docs Page - For unauthenticated users */}
        <Route path="/docs-public" element= {<DocsPage />} />


        {/* Protected Routes */}
        <Route 
          element={
            token && currentUser ? (
              <DashboardLayout currentUser={currentUser} onLogout={handleLogout} />
            ) : (
              <Navigate to="/login" replace />
            )
          }
        >
          <Route path="/repositories" element={<RepositoriesPage token={token!} />} />
          <Route path="/organizations" element={<OrganizationsPage token={token!} currentUser={currentUser!} />} />
          <Route path="/profile" element={<ProfilePage currentUser={currentUser!} token={token!} />} />
          <Route path="/docs" element={<DocsPage isEmbedded={true} />} />
        </Route>

        {/* Fallback route */}
        <Route path="*" element={<Navigate to="/" replace />} />
      </Routes>
    </HashRouter>
  );
};

export default App;