import React, { useState, useEffect, useRef } from "react";
import { Outlet, NavLink, Link } from "react-router-dom";
import { User } from "../../types";
import { AerugoIcon } from "../icons/DockerIcon";
import { UserCircleIcon } from "../icons/UserCircleIcon";
import { LogoutIcon } from "../icons/LogoutIcon";

interface DashboardLayoutProps {
  currentUser: User;
  onLogout: () => void;
}

const DashboardLayout: React.FC<DashboardLayoutProps> = ({
  currentUser,
  onLogout,
}) => {
  const [isProfileMenuOpen, setIsProfileMenuOpen] = useState(false);
  const menuRef = useRef<HTMLDivElement>(null);

  const navLinkClasses = ({ isActive }: { isActive: boolean }) =>
    `px-3 py-2 rounded-md text-sm font-medium transition-colors ${
      isActive
        ? "bg-slate-700 text-white"
        : "text-slate-300 hover:bg-slate-700/50 hover:text-white"
    }`;

  // Close menu when clicking outside
  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (menuRef.current && !menuRef.current.contains(event.target as Node)) {
        setIsProfileMenuOpen(false);
      }
    };
    document.addEventListener("mousedown", handleClickOutside);
    return () => {
      document.removeEventListener("mousedown", handleClickOutside);
    };
  }, [menuRef]);

  return (
    <div className="min-h-screen bg-slate-900 text-slate-100 font-sans">
      <header className="bg-slate-800/50 border-b border-slate-700 backdrop-blur-sm sticky top-0 z-10">
        <nav className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex items-center justify-between h-16">
            <div className="flex items-center space-x-4 lg:space-x-8">
              <Link
                to="/repositories"
                className="flex items-center space-x-3 flex-shrink-0"
              >
                <AerugoIcon className="h-7 w-7 text-indigo-500" />
                <h1 className="text-xl font-bold text-slate-50 hidden md:block">
                  Aerugo Registry
                </h1>
              </Link>
              <div className="flex items-center space-x-2 sm:space-x-4">
                <NavLink to="/repositories" className={navLinkClasses}>
                  Repositories
                </NavLink>
                <NavLink to="/organizations" className={navLinkClasses}>
                  Organizations
                </NavLink>
                <NavLink to="/docs" className={navLinkClasses}>
                  Docs
                </NavLink>
              </div>
            </div>
            <div className="flex items-center">
              <div className="relative" ref={menuRef}>
                <button
                  onClick={() => setIsProfileMenuOpen(!isProfileMenuOpen)}
                  className="flex items-center space-x-2 text-sm text-slate-300 hover:text-white focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-offset-slate-800 focus:ring-indigo-500 rounded-full"
                  aria-haspopup="true"
                  aria-expanded={isProfileMenuOpen}
                >
                  <span className="hidden sm:inline">
                    Welcome, {currentUser.username}
                  </span>
                  <UserCircleIcon className="h-8 w-8 text-slate-400" />
                </button>

                {isProfileMenuOpen && (
                  <div className="origin-top-right absolute right-0 mt-2 w-48 rounded-md shadow-lg py-1 bg-slate-700 ring-1 ring-black ring-opacity-5 z-20 animate-fade-in-scale-up">
                    <Link
                      to="/profile"
                      onClick={() => setIsProfileMenuOpen(false)}
                      className="flex items-center w-full text-left px-4 py-2 text-sm text-slate-200 hover:bg-slate-600"
                      role="menuitem"
                    >
                      <UserCircleIcon className="w-5 h-5 mr-3" />
                      My Profile
                    </Link>
                    <button
                      onClick={() => {
                        setIsProfileMenuOpen(false);
                        onLogout();
                      }}
                      className="flex items-center w-full text-left px-4 py-2 text-sm text-red-400 hover:bg-slate-600"
                      role="menuitem"
                    >
                      <LogoutIcon className="w-5 h-5 mr-3" />
                      Logout
                    </button>
                  </div>
                )}
              </div>
            </div>
          </div>
        </nav>
      </header>
      <main className="max-w-7xl mx-auto py-6 px-4 sm:px-6 lg:px-8">
        <Outlet />
      </main>
    </div>
  );
};

export default DashboardLayout;
