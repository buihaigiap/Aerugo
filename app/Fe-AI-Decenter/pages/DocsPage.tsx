import React, { useEffect, useState, useRef } from 'react';
import { Link, useLocation, useNavigate } from 'react-router-dom';
import CodeBlock from '../components/docs/CodeBlock';
import ArchitectureDiagram from '../components/docs/ArchitectureDiagram';
import { InformationCircleIcon } from '../components/icons/InformationCircleIcon';
import { ChipIcon } from '../components/icons/ChipIcon';
import { BriefcaseIcon } from '../components/icons/BriefcaseIcon';
import { ServerStackIcon } from '../components/icons/ServerStackIcon';
import { TerminalIcon } from '../components/icons/TerminalIcon';
import { ShieldExclamationIcon } from '../components/icons/ShieldExclamationIcon';
import logoImage from '../components/icons/logo.png';

const navItems = [
    { id: 'introduction', label: 'Introduction', icon: <InformationCircleIcon className="w-5 h-5" /> },
    { id: 'architecture', label: 'Architecture', icon: <ChipIcon className="w-5 h-5" /> },
    { id: 'organizations', label: 'Organizations', icon: <BriefcaseIcon className="w-5 h-5" /> },
    { id: 'repositories', label: 'Repositories', icon: <ServerStackIcon className="w-5 h-5" /> },
    { id: 'docker-usage', label: 'Using Docker', icon: <TerminalIcon className="w-5 h-5" /> },
    { id: 'tos', label: 'Terms of Service', icon: <ShieldExclamationIcon className="w-5 h-5" /> },
];

const sectionStyles: { [key: string]: { bg: string; accent: string; icon: string } } = {
    introduction: { bg: 'from-slate-800/80 to-indigo-900/40', accent: 'border-indigo-400', icon: 'text-indigo-400' },
    architecture: { bg: 'from-slate-800/80 to-sky-900/40', accent: 'border-sky-400', icon: 'text-sky-400' },
    organizations: { bg: 'from-slate-800/80 to-teal-900/40', accent: 'border-teal-400', icon: 'text-teal-400' },
    repositories: { bg: 'from-slate-800/80 to-blue-900/40', accent: 'border-blue-400', icon: 'text-blue-400' },
    'docker-usage': { bg: 'from-slate-800/80 to-cyan-900/40', accent: 'border-cyan-400', icon: 'text-cyan-400' },
    tos: { bg: 'from-slate-800/80 to-purple-900/40', accent: 'border-purple-400', icon: 'text-purple-400' },
};

interface DocsPageProps {
    /**
     * When true, the component renders without its own header and page-level layout,
     * making it suitable for embedding within another layout like the dashboard.
     * @default false
     */
    isEmbedded?: boolean;
}


const DocsPage: React.FC<DocsPageProps> = ({ isEmbedded = false }) => {
    const location = useLocation();
    const navigate = useNavigate();
    const [activeSection, setActiveSection] = useState(navItems[0].id);
    const isInitialMount = useRef(true);

    const headerHeight = isEmbedded ? 80 : 80; // dashboard header + padding vs standalone header

    // Effect for observing sections during manual scroll and updating active state
    useEffect(() => {
        const observer = new IntersectionObserver(
            (entries) => {
                entries.forEach((entry) => {
                    if (entry.isIntersecting) {
                        setActiveSection(entry.target.id);
                    }
                });
            },
            { rootMargin: '-30% 0px -70% 0px', threshold: 0 }
        );

        navItems.forEach((item) => {
            const element = document.getElementById(item.id);
            if (element) {
                observer.observe(element);
            }
        });

        return () => {
            navItems.forEach((item) => {
                const element = document.getElementById(item.id);
                if (element) {
                    observer.unobserve(element);
                }
            });
        };
    }, []);

    // Effect for handling scrolling when the URL hash changes (from clicks or direct links)
    useEffect(() => {
        const hash = location.hash.substring(1);
        if (hash) {
            const element = document.getElementById(hash);
            if (element) {
                const elementPosition = element.getBoundingClientRect().top;
                const offsetPosition = elementPosition + window.pageYOffset - headerHeight;

                const behavior = isInitialMount.current ? 'auto' : 'smooth';

                setTimeout(() => {
                    window.scrollTo({
                        top: offsetPosition,
                        behavior,
                    });
                    isInitialMount.current = false;
                }, 100);
            }
        }
    }, [location.hash, headerHeight]);

    const handleNavClick = (e: React.MouseEvent<HTMLAnchorElement>, id: string) => {
        e.preventDefault();
        setActiveSection(id);
        navigate(`#${id}`, { replace: true });
    };

    const REGISTRY_HOST = 'aerugo.io';
    const currentStyle = sectionStyles[activeSection] || sectionStyles.introduction;

    const wrapperClass = isEmbedded
        ? "text-slate-200 font-sans"
        : "min-h-screen bg-slate-900 bg-gradient-to-br from-slate-900 via-slate-900 to-indigo-950/30 text-slate-200 font-sans";

    const MainWrapper = isEmbedded ? 'div' : 'main';
    const mainWrapperProps = isEmbedded ? {} : { className: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8" };

    const headerTopPadding = isEmbedded ? "pt-0" : "pt-16";
    const titleClass = isEmbedded ? "text-3xl font-bold" : "text-4xl sm:text-5xl font-extrabold";
    const subTitleMargin = isEmbedded ? "mt-1" : "mt-4";
    const sidebarTopClass = isEmbedded ? "lg:top-20" : "lg:top-24";
    const sectionScrollMarginClass = isEmbedded ? "scroll-mt-20" : "scroll-mt-24";

    return (
        <div className={wrapperClass}>
            {!isEmbedded && (
                <header className="py-4 px-4 sm:px-6 lg:px-8 bg-slate-900/80 backdrop-blur-lg sticky top-0 z-40 border-b border-slate-800">
                    <nav className="flex items-center justify-between max-w-7xl mx-auto">
                        <Link to="/" className="flex items-center space-x-3 group">
                            <img src={logoImage} alt="Aerugo Logo" className="h-[100px] w-[100px] transition-transform duration-300 group-hover:scale-110" />
                            <span className="brand-font text-3xl font-bold tracking-wider bg-gradient-to-r from-slate-100 to-indigo-300 text-transparent bg-clip-text">Aerugo</span>
                        </Link>
                        <div className="flex items-center gap-x-2 sm:gap-x-4">
                            <Link to="/login" className="font-semibold text-slate-300 hover:text-white focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 focus:ring-offset-slate-900 rounded px-3 py-2 transition-colors">
                                Sign In
                            </Link>
                            <Link to="/register" className="font-semibold bg-indigo-600 hover:bg-indigo-700 text-white focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 focus:ring-offset-slate-900 rounded px-4 py-2 transition-colors">
                                Sign Up
                            </Link>
                        </div>
                    </nav>
                </header>
            )}

            <MainWrapper {...mainWrapperProps}>
                <div className={`text-center ${headerTopPadding} pb-12 border-b border-slate-800`}>
                    <h1 className={`${titleClass} tracking-tight bg-gradient-to-r from-slate-100 to-indigo-300 text-transparent bg-clip-text`}>Documentation</h1>
                    <p className={`${subTitleMargin} max-w-2xl mx-auto text-lg text-slate-400`}>Everything you need to know to get started with Aerugo.</p>
                </div>

                <div className="flex flex-col lg:flex-row gap-12 lg:gap-16 py-12">
                    {/* Sidebar */}
                    <aside className={`lg:w-64 lg:flex-shrink-0 lg:sticky ${sidebarTopClass} self-start z-30`}>
                        <div className={`p-4 border border-slate-700/80 rounded-xl bg-gradient-to-br transition-colors duration-500 ${currentStyle.bg}`}>
                            <h3 className="text-sm font-semibold tracking-wider text-slate-300 uppercase mb-3 px-2">On this page</h3>
                            <nav>
                                <ul className="space-y-0.5">
                                    {navItems.map(item => {
                                        const isActive = activeSection === item.id;
                                        return (
                                            <li key={item.id}>
                                                <a
                                                    href={`#${item.id}`}
                                                    onClick={(e) => handleNavClick(e, item.id)}
                                                    className={`flex items-center gap-x-3 py-2 px-3 rounded-md text-sm font-medium transition-all duration-200 border-l-4 ${isActive
                                                        ? `${sectionStyles[item.id]?.accent || 'border-indigo-400'} bg-white/5 text-slate-50`
                                                        : 'border-transparent text-slate-400 hover:bg-white/5 hover:text-slate-200'
                                                        }`}
                                                >
                                                    <span className={`transition-colors ${isActive ? sectionStyles[item.id]?.icon || 'text-indigo-400' : 'text-slate-500'}`}>
                                                        {item.icon}
                                                    </span>
                                                    <span>{item.label}</span>
                                                </a>
                                            </li>
                                        );
                                    })}
                                </ul>
                            </nav>
                        </div>
                    </aside>

                    {/* Main content */}
                    <div className="flex-1 min-w-0">
                        <div className="prose-custom space-y-12">
                            <section id="introduction" className={sectionScrollMarginClass}>
                                <h1>Introduction</h1>
                                <p>
                                    Welcome to Aerugo! This guide will walk you through managing your container images using this web interface. Here you can create organizations for your teams, manage your repositories, and control access for your members.
                                </p>
                                <blockquote>
                                    <p><strong>Note:</strong> This service is currently in a beta phase. APIs and features may change.</p>
                                </blockquote>
                            </section>

                            <hr />

                            <section id="architecture" className={sectionScrollMarginClass}>
                                <h2>System Architecture</h2>
                                <p>
                                    Aerugo is designed as a distributed, highly available system. It consists of stateless application nodes written in Rust that connect to a set of robust backend services. This architecture ensures scalability and resilience.
                                </p>
                                <ArchitectureDiagram />
                            </section>

                            <hr />

                            <section id="organizations" className={sectionScrollMarginClass}>
                                <h2>Managing Organizations</h2>
                                <p>An organization acts as a workspace for your team. It contains repositories and members. You can create multiple organizations to separate projects or teams.</p>
                                <h3>Creating an Organization</h3>
                                <ol>
                                    <li>Navigate to the <Link to="/organizations">Organizations</Link> page after signing in.</li>
                                    <li>Click the "Create New" button.</li>
                                    <li>
                                        Fill in the form:
                                        <ul>
                                            <li><strong>Display Name:</strong> The name that will be shown throughout the UI (e.g., "My Awesome Team").</li>
                                            <li><strong>Organization Name (URL):</strong> A unique, URL-friendly identifier for your organization. This is used in docker commands (e.g., "my-awesome-team"). It must be lowercase and can only contain letters, numbers, and hyphens.</li>
                                        </ul>
                                    </li>
                                    <li>Click "Create Organization" to finish.</li>
                                </ol>
                                <h3>Managing Members & Roles</h3>
                                <p>Once you've selected an organization, you can manage its members from the "Members" tab. Roles include:</p>
                                <ul>
                                    <li><strong>Owner:</strong> Full administrative control, including deleting the organization.</li>
                                    <li><strong>Admin:</strong> Can manage repositories and members (except other Owners).</li>
                                    <li><strong>Member:</strong> Can view and (depending on repository settings) push/pull images.</li>
                                </ul>
                            </section>

                            <hr />

                            <section id="repositories" className={sectionScrollMarginClass}>
                                <h2>Managing Repositories</h2>
                                <p>A repository is a collection of related container images, identified by different tags (e.g., <code>:latest</code>, <code>:v1.0</code>).</p>
                                <h3>Creating a Repository</h3>
                                <p>After selecting an organization on the <Link to="/repositories">Repositories</Link> page, click "Create Repository". You'll name your repository and choose its visibility.</p>
                                <h3>Public vs. Private Repositories</h3>
                                <ul>
                                    <li><strong>Public:</strong> Anyone, even unauthenticated users, can pull images.</li>
                                    <li><strong>Private:</strong> Only members of the organization can pull images.</li>
                                </ul>
                            </section>

                            <hr />

                            <section id="docker-usage" className={sectionScrollMarginClass}>
                                <h2>Using Docker</h2>
                                <p>To push and pull images, you'll use the Docker command-line tool. For more details, check out the official GitHub repository: <a href="https://github.com/AI-Decenter/Aerugo" target="_blank" rel="noopener noreferrer">https://github.com/AI-Decenter/Aerugo</a></p>

                                <h3>1. Log In to the Registry</h3>
                                <p>First, log in to the Aerugo registry using your account credentials. The interactive method is recommended for security.</p>
                                <CodeBlock code={`docker login ${REGISTRY_HOST}`} language="bash" />
                                <p>You will be prompted for your username and password.</p>
                                <p>For automated environments, you can use a non-interactive login. Be aware of the security implications of exposing your password in your command history.</p>
                                <CodeBlock code={`docker login -u your-username -p your-password ${REGISTRY_HOST}`} language="bash" />

                                <h3>2. Push an Image</h3>
                                <p>To push an image, you first need to tag it with the full registry path: <code>{`${REGISTRY_HOST}/[organization-name]/[repository-name]:[tag]`}</code>. Remember to use the URL-friendly organization name (e.g., "my-awesome-team"), not the display name.</p>
                                <CodeBlock code={`docker tag my-local-image:latest ${REGISTRY_HOST}/my-awesome-team/my-app:v1.0`} language="bash" />
                                <p>After tagging, push the image to the registry.</p>
                                <CodeBlock code={`docker push ${REGISTRY_HOST}/my-awesome-team/my-app:v1.0`} language="bash" />

                                <h3>3. Pull an Image</h3>
                                <p>To pull the image on another machine, use the <code>docker pull</code> command with the same full path.</p>
                                <CodeBlock code={`docker pull ${REGISTRY_HOST}/my-awesome-team/my-app:v1.0`} language="bash" />

                                <h3>4. Run a Container</h3>
                                <p>After pulling the image, you can run it as a container. The <code>--rm</code> flag is useful for automatically removing the container when it exits.</p>
                                <CodeBlock code={`docker run --rm ${REGISTRY_HOST}/my-awesome-team/my-app:v1.0`} language="bash" />
                            </section>

                            <hr />

                            <section id="tos" className={sectionScrollMarginClass}>
                                <h2>Terms of Service</h2>
                                <p>Last Updated: {new Date().toLocaleDateString()}</p>

                                <h3>1. Acceptance of Terms</h3>
                                <p>By accessing or using the Aerugo service ("Service"), you agree to be bound by these Terms of Service ("Terms").</p>

                                <h3>2. User Content & Conduct</h3>
                                <p>You retain full ownership of any content you upload. You are responsible for safeguarding your account and for all content you upload, ensuring it does not violate any laws or third-party rights.</p>

                                <h3>3. Termination</h3>
                                <p>We may terminate or suspend your access to our Service immediately, without prior notice or liability, for any reason, including breach of these Terms.</p>

                                <h3>4. Disclaimer & Limitation of Liability</h3>
                                <p>The Service is provided "AS IS" without warranties. We are not liable for any damages or losses resulting from your use of the service.</p>
                            </section>
                        </div>
                    </div>
                </div>
            </MainWrapper>
            <style>{`
                .prose-custom h1 {
                    font-size: 2rem;
                    line-height: 2.5rem;
                    font-weight: 800;
                    letter-spacing: -0.025em;
                    color: #f8fafc;
                    margin-bottom: 1rem;
                }
                .prose-custom h2 {
                    font-size: 1.5rem;
                    line-height: 2rem;
                    font-weight: 700;
                    color: #f1f5f9;
                    position: relative;
                    padding-bottom: 0.75rem;
                    margin-top: 2rem;
                    margin-bottom: 1.5rem;
                }
                .prose-custom h2::after {
                    content: '';
                    position: absolute;
                    left: 0;
                    bottom: 0;
                    width: 3rem;
                    height: 2px;
                    background: linear-gradient(to right, #6366f1, #818cf8);
                    border-radius: 1px;
                }
                .prose-custom h3 {
                    font-size: 1.25rem;
                    line-height: 1.75rem;
                    font-weight: 600;
                    color: #e2e8f0;
                    margin-top: 2rem;
                }
                .prose-custom h4 {
                    font-size: 1.1rem;
                    line-height: 1.5rem;
                    font-weight: 600;
                    color: #e2e8f0;
                    margin-top: 1.5rem;
                }
                .prose-custom p,
                .prose-custom li {
                    font-size: 1rem;
                    line-height: 1.75;
                    color: #cbd5e1;
                }
                .prose-custom a {
                    color: #818cf8;
                    font-weight: 500;
                    text-decoration: none;
                    transition: color 0.2s ease-in-out;
                }
                .prose-custom a:hover {
                    color: #a7aefe;
                    text-decoration: underline;
                }
                .prose-custom strong {
                    color: #f1f5f9;
                    font-weight: 600;
                }
                .prose-custom ol,
                .prose-custom ul {
                    padding-left: 0;
                    margin-top: 1.5rem;
                    margin-bottom: 1.5rem;
                    display: flex;
                    flex-direction: column;
                    gap: 0.75rem;
                }
                .prose-custom li {
                    position: relative;
                    padding-left: 1.75rem;
                }
                .prose-custom ul > li::before {
                    content: '';
                    position: absolute;
                    left: 0.25rem;
                    top: 0.6rem;
                    width: 0.5rem;
                    height: 0.5rem;
                    background-color: #6366f1;
                    border-radius: 50%;
                }
                 .prose-custom ol {
                    list-style: none;
                    counter-reset: item;
                 }
                 .prose-custom ol > li::before {
                    content: counter(item);
                    counter-increment: item;
                    position: absolute;
                    left: 0;
                    top: 0.25rem;
                    width: 1.25rem;
                    height: 1.25rem;
                    border-radius: 50%;
                    background-color: #3730a3;
                    color: #e0e7ff;
                    font-size: 0.75rem;
                    font-weight: bold;
                    display: inline-flex;
                    align-items: center;
                    justify-content: center;
                 }
                .prose-custom code {
                    color: #c7d2fe;
                    background-color: #374151;
                    padding: 0.125rem 0.375rem;
                    border-radius: 0.25rem;
                    font-size: 0.9em;
                }
                .prose-custom blockquote {
                    border-left: 4px solid #4f46e5;
                    padding: 1rem 1.5rem;
                    background-color: rgba(79, 70, 229, 0.1);
                    color: #c7d2fe;
                    margin-left: 0;
                    margin-right: 0;
                    border-radius: 0 0.5rem 0.5rem 0;
                }
                .prose-custom blockquote p {
                    color: #e0e7ff;
                    margin: 0;
                }
                 .prose-custom hr {
                    border: 0;
                    height: 1px;
                    background: linear-gradient(to right, transparent, #334155, transparent);
                    margin: 3rem 0;
                }
            `}</style>
        </div>
    );
};

export default DocsPage;