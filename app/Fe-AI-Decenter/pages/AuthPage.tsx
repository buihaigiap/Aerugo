

// FIX: Imported useState to resolve 'Cannot find name' error.
import React, { useRef, useEffect, useState } from 'react';
import { Link } from 'react-router-dom';
import { BriefcaseIcon } from '../components/icons/BriefcaseIcon';
import { ShieldCheckIcon } from '../components/icons/ShieldCheckIcon';
import { CodeBracketIcon } from '../components/icons/CodeBracketIcon';
import { ServerStackIcon } from '../components/icons/ServerStackIcon';
import { UsersIcon } from '../components/icons/UsersIcon';
import { CloudIcon } from '../components/icons/CloudIcon';
import { RocketLaunchIcon } from '../components/icons/RocketLaunchIcon';
import { GithubIcon } from '../components/icons/GithubIcon';
import { TwitterIcon } from '../components/icons/TwitterIcon';
import { DiscordIcon } from '../components/icons/DiscordIcon';
import AnimatedParticleBackground from '../components/AnimatedParticleBackground';
import { ChipIcon } from '../components/icons/ChipIcon';

const AuthPage: React.FC = () => {
    const introductionSectionRef = useRef<HTMLDivElement>(null);
    const cardsContainerRef = useRef<HTMLDivElement>(null);

    const handleDocsClick = () => {
        introductionSectionRef.current?.scrollIntoView({ behavior: 'smooth' });
    };

    useEffect(() => {
        // Intersection observer for cards
        const observer = new IntersectionObserver((entries) => {
            entries.forEach(entry => {
                if (entry.isIntersecting) {
                    entry.target.classList.add('is-visible');
                    observer.unobserve(entry.target);
                }
            });
        }, { threshold: 0.1 });

        if (cardsContainerRef.current) {
            observer.observe(cardsContainerRef.current);
        }

        return () => {
            if (cardsContainerRef.current) {
                observer.unobserve(cardsContainerRef.current);
            }
        };
    }, []);

    const coreConcepts = [
        {
            icon: <BriefcaseIcon className="w-4 h-4 text-sky-400" />,
            title: 'Organizations',
            description: 'Create dedicated workspaces for your teams or projects. Each organization is a self-contained unit with its own members and repositories.'
        },
        {
            icon: <ServerStackIcon className="w-4 h-4 text-sky-400" />,
            title: 'Repositories',
            description: 'Store and manage your container images. Repositories belong to organizations, inheriting their access policies.'
        },
        {
            icon: <UsersIcon className="w-4 h-4 text-sky-400" />,
            title: 'Members & Roles',
            description: 'Invite users to your organizations with role-based access control (Owner, Admin, Member) to ensure security and proper governance.'
        }
    ];

    return (
        <div className="min-h-screen bg-slate-900 text-slate-100 font-sans overflow-x-hidden">
            {/* Header */}
            <header className="py-4 px-4 sm:px-6 lg:px-8 bg-slate-900/80 backdrop-blur-lg sticky top-0 z-30 border-b border-slate-800">
                <nav className="flex items-center justify-between">
                    <Link to="/" className="flex items-center space-x-3 group">
                        <img src="./components/icons/logo.png" alt="Aerugo Logo" className="h-[100px] w-[100px] transition-transform duration-300 group-hover:scale-110" />
                        <span className="brand-font text-3xl font-bold tracking-wider bg-gradient-to-r from-slate-100 to-indigo-300 text-transparent bg-clip-text">Aerugo</span>
                    </Link>
                    <div className="flex items-center gap-x-2 sm:gap-x-4">
                        <button
                            onClick={handleDocsClick}
                            className="font-semibold text-slate-300 hover:text-white focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 focus:ring-offset-slate-900 rounded px-3 py-2 transition-colors"
                        >
                            Introduction
                        </button>
                        <Link
                            to="/docs-public"
                            className="font-semibold text-slate-300 hover:text-white focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 focus:ring-offset-slate-900 rounded px-3 py-2 transition-colors"
                        >
                            Docs
                        </Link>
                        <Link
                            to="/login"
                            className="font-semibold text-slate-300 hover:text-white focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 focus:ring-offset-slate-900 rounded px-3 py-2 transition-colors"
                        >
                            Sign In
                        </Link>
                    </div>
                </nav>
            </header>

            {/* Hero Section */}
            <main className="relative pt-20 pb-16 sm:pt-24 lg:pt-32 text-center">
                <AnimatedParticleBackground />
                <div className="relative z-10 max-w-4xl mx-auto px-4">
                    <h1 className="text-4xl sm:text-5xl lg:text-6xl font-extrabold tracking-tight bg-gradient-to-r from-slate-100 to-indigo-300 text-transparent bg-clip-text animate-shimmer">
                        The Modern, Secure Container Registry
                    </h1>
                    <p className="mt-6 max-w-2xl mx-auto text-lg text-slate-400">
                        Streamline your development workflow with a private, scalable, and easy-to-use registry for all your container images.
                    </p>
                    <div className="mt-10">
                        <a
                            href="https://decenter.ai"
                            target="_blank"
                            rel="noopener noreferrer"
                            className="inline-flex items-center justify-center px-8 py-4 font-semibold text-white bg-gradient-to-r from-sky-500 to-indigo-600 rounded-full shadow-lg transition-all duration-300 ease-in-out transform hover:scale-105 hover:shadow-sky-400/50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-offset-slate-900 focus:ring-sky-500 animate-subtle-pulse"
                        >
                            <ChipIcon className="w-6 h-6 mr-3" />
                            <span>Explore Decenter AI</span>
                        </a>
                    </div>
                </div>
            </main>

            {/* Introduction Section */}
            <section ref={introductionSectionRef} id="introduction" className="relative py-24 sm:py-32 bg-slate-900/70 scroll-mt-20 overflow-hidden">
                {/* Animated Background */}
                <div className="absolute inset-0 z-0">
                    {/* Pulsing Core */}
                    <div className="absolute inset-0 bg-indigo-950/30 [mask-image:radial-gradient(ellipse_at_center,black_20%,transparent_70%)] animate-pulse-glow"></div>
                    {/* Scrolling Grid */}
                    <div
                        className="absolute inset-0 opacity-[0.05] animate-scroll-grid"
                        style={{
                            backgroundImage: 'linear-gradient(rgba(203, 213, 225, 0.5) 1px, transparent 1px), linear-gradient(to right, rgba(203, 213, 225, 0.5) 1px, transparent 1px)',
                            backgroundSize: '4rem 4rem',
                        }}
                    />
                    {/* Running Lights */}
                    <div className="absolute inset-0 animate-running-lights-vertical"></div>
                    <div className="absolute inset-0 animate-running-lights-horizontal"></div>
                </div>

                <div className="relative z-10 max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                    <div className="text-center">
                        <h2 className="text-4xl sm:text-5xl font-extrabold tracking-tight bg-gradient-to-r from-teal-300 via-sky-400 to-indigo-400 text-transparent bg-clip-text animate-shimmer">
                            Built for the Future of Development
                        </h2>
                        <p className="mt-6 max-w-3xl mx-auto text-lg text-slate-300/90">
                            Aerugo is a next-generation, distributed, and multi-tenant container registry built with Rust. Designed for high performance and scalability, leveraging an S3-compatible object storage backend.
                        </p>
                    </div>
                    <div ref={cardsContainerRef} className="mt-20 grid gap-8 md:grid-cols-2 lg:grid-cols-4 feature-card-container">
                        {techFeatures.map((feature, index) => (
                            <TechFeatureCard
                                key={feature.title}
                                icon={feature.icon}
                                title={feature.title}
                                description={feature.description}
                                animationDelay={`${150 * index}ms`}
                            />
                        ))}
                    </div>
                </div>
            </section>

            {/* Workflow Section */}
            <section className="pt-12 sm:pt-16 pb-24 sm:pb-32">
                <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                    <div className="text-center">
                        <h2 className="text-3xl sm:text-4xl font-extrabold tracking-tight bg-gradient-to-r from-slate-100 to-indigo-300 text-transparent bg-clip-text animate-shimmer">
                            Designed for a Modern Workflow
                        </h2>
                        <p className="mt-4 max-w-2xl mx-auto text-lg text-slate-400">
                            Everything you need to build, secure, and deploy your applications at scale.
                        </p>
                    </div>
                    <div className="mt-20 grid grid-cols-1 lg:grid-cols-3 lg:gap-8">
                        <div className="lg:col-span-2">
                            <InteractiveWorkflowCard
                                {...workflowFeatures[0]}
                                className="h-full"
                            >
                                <AnimatedCodeSnippet />
                            </InteractiveWorkflowCard>
                        </div>
                        <div className="grid gap-8 mt-8 lg:mt-0">
                            <InteractiveWorkflowCard {...workflowFeatures[1]} />
                            <InteractiveWorkflowCard {...workflowFeatures[2]} />
                        </div>
                    </div>
                </div>
            </section>

            {/* Core Concepts Section */}
            <section className="relative py-24 sm:py-32 bg-slate-900/70 overflow-hidden">
                <div className="absolute inset-0 z-0 opacity-[0.03] animate-bg-grid-flow" style={{ backgroundSize: '2.5rem 2.5rem' }}></div>
                <div className="relative max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                    <div className="grid grid-cols-1 lg:grid-cols-2 gap-16 items-center">
                        <div className="text-center lg:text-left">
                            <h2 className="text-4xl sm:text-5xl font-extrabold tracking-tight bg-gradient-to-r from-teal-300 via-sky-400 to-indigo-400 text-transparent bg-clip-text animate-shimmer">
                                Powerful Core Concepts
                            </h2>
                            <p className="mt-6 text-lg text-slate-300/90">
                                Aerugo is built around a flexible, hierarchical structure that makes managing your software supply chain intuitive and secure.
                            </p>
                            <div className="mt-10 relative text-left">
                                <div className="absolute left-4 top-4 bottom-4 w-px bg-slate-700/50" aria-hidden="true">
                                    <div className="absolute inset-y-0 left-0 w-full animate-flow-line" />
                                </div>
                                <div className="space-y-12">
                                    {coreConcepts.map((concept, index) => (
                                        <div key={index} className="relative pl-12">
                                            <div className="absolute left-0 top-0 w-8 h-8 bg-slate-900 rounded-full flex items-center justify-center border-2 border-sky-500 shadow-[0_0_15px_rgba(14,165,233,0.4)] transition-all duration-300 hover:scale-110 hover:shadow-[0_0_25px_rgba(14,165,233,0.6)]">
                                                {concept.icon}
                                            </div>
                                            <div className="pl-2">
                                                <h3 className="font-semibold text-slate-50 text-lg">{concept.title}</h3>
                                                <p className="text-slate-400 mt-1">{concept.description}</p>
                                            </div>
                                        </div>
                                    ))}
                                </div>
                            </div>
                        </div>
                        <div className="min-h-[300px] sm:min-h-[400px] flex items-center justify-center">
                            <DynamicCoreVisual />
                        </div>
                    </div>
                </div>
            </section>


            {/* Footer */}
            <footer className="relative bg-slate-900/70 border-t border-slate-800/50 overflow-hidden">
                <div className="absolute inset-0 z-0">
                    <div
                        className="absolute inset-0 opacity-[0.03] animate-bg-grid-flow"
                        style={{
                            backgroundImage: 'linear-gradient(rgba(203, 213, 225, 1) 1px, transparent 1px), linear-gradient(to right, rgba(203, 213, 225, 1) 1px, transparent 1px)',
                            backgroundSize: '3rem 3rem'
                        }}
                    />
                </div>
                <div className="relative max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-16">
                    <div className="grid grid-cols-2 md:grid-cols-4 lg:grid-cols-5 gap-8 text-sm">
                        {/* Brand Column */}
                        <div className="col-span-2 md:col-span-4 lg:col-span-2">
                            <div className="flex items-center space-x-3">
                                <img src="./components/icons/logo.png" alt="Aerugo Logo" className="h-[100px] w-[100px]" />
                                <span className="brand-font text-2xl font-bold text-slate-50">Aerugo</span>
                            </div>
                            <p className="mt-4 text-slate-400 max-w-xs">
                                A modern, secure, and performant container registry built for developers and teams.
                            </p>
                        </div>

                        {/* Links Columns */}
                        <div>
                            <h3 className="font-semibold text-slate-100 tracking-wider uppercase">Product</h3>
                            <ul className="mt-4 space-y-3">
                                <li>
                                    <button onClick={handleDocsClick} className="text-slate-400 hover:text-indigo-300 transition-colors">
                                        Introduction
                                    </button>
                                </li>
                                <li>
                                    <Link to="/repositories" className="text-slate-400 hover:text-indigo-300 transition-colors">
                                        Repositories
                                    </Link>
                                </li>
                                <li>
                                    <Link to="/organizations" className="text-slate-400 hover:text-indigo-300 transition-colors">
                                        Organizations
                                    </Link>
                                </li>
                            </ul>
                        </div>

                        <div>
                            <h3 className="font-semibold text-slate-100 tracking-wider uppercase">Resources</h3>
                            <ul className="mt-4 space-y-3">
                                <li>
                                    <Link to="/docs-public" className="text-slate-400 hover:text-indigo-300 transition-colors">
                                        Docs
                                    </Link>
                                </li>
                                <li>
                                    <a href="https://github.com/AI-Decenter/Aerugo" target="_blank" rel="noopener noreferrer" className="text-slate-400 hover:text-indigo-300 transition-colors">
                                        GitHub
                                    </a>
                                </li>
                                <li>
                                    <a href="#" className="text-slate-400 hover:text-indigo-300 transition-colors">
                                        API Status
                                    </a>
                                </li>
                            </ul>
                        </div>
                        <div>
                            <h3 className="font-semibold text-slate-100 tracking-wider uppercase">Legal</h3>
                            <ul className="mt-4 space-y-3">
                                <li>
                                    <Link to="/docs-public#tos" className="text-slate-400 hover:text-indigo-300 transition-colors">
                                        Terms of Service
                                    </Link>
                                </li>
                                <li>
                                    <a href="#" className="text-slate-400 hover:text-indigo-300 transition-colors">
                                        Privacy Policy
                                    </a>
                                </li>
                            </ul>
                        </div>
                    </div>
                    <div className="mt-16 pt-8 border-t border-slate-800/50 flex flex-col sm:flex-row justify-between items-center text-sm text-slate-500">
                        <p>&copy; {new Date().getFullYear()} Aerugo. All rights reserved.</p>
                        <div className="flex items-center space-x-6 mt-4 sm:mt-0">
                            <a href="https://github.com/AI-Decenter/Aerugo" target="_blank" rel="noopener noreferrer" className="text-slate-500 hover:text-indigo-400 transition-colors" aria-label="GitHub">
                                <GithubIcon className="w-6 h-6" />
                            </a>
                            <a href="#" className="text-slate-500 hover:text-indigo-400 transition-colors" aria-label="Twitter">
                                <TwitterIcon className="w-6 h-6" />
                            </a>
                            <a href="#" className="text-slate-500 hover:text-indigo-400 transition-colors" aria-label="Discord">
                                <DiscordIcon className="w-6 h-6" />
                            </a>
                        </div>
                    </div>
                </div>
            </footer>
        </div>
    );
};

const DynamicCoreVisual = () => {
    return (
        <div className="relative w-full h-full min-h-[300px] sm:min-h-[400px] flex items-center justify-center">
            {/* Replace canvas with an image tag for the GIF */}
            <img
                src="https://media3.giphy.com/media/v1.Y2lkPTc5MGI3NjExM3h4MDA3eXpqZ3FzbjVqYzJ0dTVvNnoxbGM4eGNpNjdtdW9mNmY0eiZlcD12MV9pbnRlcm5hbF9naWZfYnlfaWQmY3Q9Zw/9tA6H1madRvUc/giphy.gif"
                alt="Rotating tech globe visualization"
                className="absolute inset-0 w-full h-full object-contain opacity-60 mix-blend-screen transform scale-110"
            />
        </div>
    );
};

const techFeatures = [
    {
        icon: <ServerStackIcon className="h-8 w-8 text-sky-300" />,
        title: "Distributed & Highly Available",
        description: "Designed to run in a clustered environment with no single point of failure.",
    },
    {
        icon: <UsersIcon className="h-8 w-8 text-sky-300" />,
        title: "Multi-tenancy",
        description: "First-class support for users and organizations with granular access control.",
    },
    {
        icon: <CloudIcon className="h-8 w-8 text-sky-300" />,
        title: "S3-Compatible Backend",
        description: "Uses any S3-compatible object storage for durability and infinite scalability.",
    },
    {
        icon: <RocketLaunchIcon className="h-8 w-8 text-sky-300" />,
        title: "Written in Rust",
        description: "Provides memory safety, concurrency, and performance for a secure, efficient core.",
    },
];

const AnimatedCodeSnippet = () => {
    const codeLines = [
        '# repository-policy.yaml',
        'repository: a-team/webapp-frontend',
        '',
        '# Access rules define team and user permissions.',
        'permissions:',
        "  - team: 'frontend-developers'",
        "    level: 'read-write'",
        "  - team: 'platform-engineering'",
        "    level: 'read-only'",
    ];
    const fullCode = codeLines.join('\n');
    const [displayedCode, setDisplayedCode] = useState('');

    useEffect(() => {
        let currentIndex = 0;
        let timeoutId: number;

        const type = () => {
            if (currentIndex < fullCode.length) {
                setDisplayedCode(prev => fullCode.substring(0, currentIndex + 1));
                currentIndex++;
                timeoutId = window.setTimeout(type, Math.random() * 20 + 10);
            } else {
                timeoutId = window.setTimeout(() => {
                    setDisplayedCode('');
                    currentIndex = 0;
                    type();
                }, 4000);
            }
        };

        timeoutId = window.setTimeout(type, 500);

        return () => clearTimeout(timeoutId);
    }, [fullCode]);

    return (
        <div className="mt-6 w-full h-80 bg-slate-900/70 rounded-lg border border-slate-700 font-mono text-sm text-indigo-300 overflow-hidden flex flex-col">
            <div className="flex-shrink-0 bg-slate-800/80 px-4 py-2 flex items-center gap-2 border-b border-slate-700">
                <div className="w-3 h-3 rounded-full bg-red-500"></div>
                <div className="w-3 h-3 rounded-full bg-yellow-500"></div>
                <div className="w-3 h-3 rounded-full bg-green-500"></div>
            </div>
            <div className="p-4 overflow-hidden">
                <pre className="whitespace-pre-wrap">
                    <code>
                        {displayedCode}
                        <span className="inline-block w-2.5 h-4 bg-indigo-300 animate-blinking-cursor ml-0.5"></span>
                    </code>
                </pre>
            </div>
        </div>
    );
};


const workflowFeatures = [
    {
        icon: <BriefcaseIcon className="h-8 w-8 text-indigo-300" />,
        title: "Centralized Organizations",
        description: "Group repositories under organizations for powerful, team-based access control and management.",
        bgClass: 'animate-bg-grid-flow',
    },
    {
        icon: <ShieldCheckIcon className="h-8 w-8 text-indigo-300" />,
        title: "Robust Security",
        description: "Secure your software supply chain with private repositories and granular, role-based permissions.",
        bgClass: 'animate-bg-shift'
    },
    {
        icon: <CodeBracketIcon className="h-8 w-8 text-indigo-300" />,
        title: "Seamless Integration",
        description: "Fully compatible with the Docker CLI and designed to integrate perfectly with your existing CI/CD pipelines.",
        bgClass: 'animate-bg-lines'
    }
];


interface TechFeatureCardProps {
    icon: React.ReactNode;
    title: string;
    description: string;
    animationDelay: string;
}

const TechFeatureCard: React.FC<TechFeatureCardProps> = ({ icon, title, description, animationDelay }) => {
    return (
        <div
            className="feature-card group relative bg-transparent rounded-xl transition-all duration-300 ease-out opacity-0"
            style={{ animationDelay }}
        >
            <div
                className="relative h-full bg-slate-900/50 backdrop-blur-sm rounded-xl border border-slate-700 overflow-hidden transition-all duration-300 group-hover:bg-slate-800/60 group-hover:border-slate-600 animate-bg-pan"
            >
                <div className="relative z-10 p-6 h-full flex flex-col transition-transform duration-300 group-hover:scale-105">
                    <div className="mb-4 inline-block p-3 bg-slate-800 rounded-lg border border-slate-700 transition-colors duration-300 group-hover:bg-slate-700">
                        {icon}
                    </div>
                    <h3 className="text-lg font-semibold text-slate-50">{title}</h3>
                    <p className="mt-2 text-sm text-slate-400 flex-grow">{description}</p>
                </div>
            </div>
        </div>
    );
};


interface InteractiveWorkflowCardProps {
    icon: React.ReactNode;
    title: string;
    description: string;
    bgClass: string;
    className?: string;
    children?: React.ReactNode;
}

const InteractiveWorkflowCard: React.FC<InteractiveWorkflowCardProps> = ({ icon, title, description, bgClass, className = '', children }) => {
    const cardRef = useRef<HTMLDivElement>(null);

    const handleMouseMove = (e: React.MouseEvent<HTMLDivElement>) => {
        const card = cardRef.current;
        if (!card) return;
        const rect = card.getBoundingClientRect();
        const x = e.clientX - rect.left;
        const y = e.clientY - rect.top;
        card.style.setProperty('--mouse-x', `${x}px`);
        card.style.setProperty('--mouse-y', `${y}px`);
    };

    return (
        <div
            ref={cardRef}
            onMouseMove={handleMouseMove}
            className={`group relative p-8 bg-slate-900 rounded-xl border border-slate-700/80 transition-all duration-300 hover:border-indigo-500/50 hover:bg-slate-900/80 overflow-hidden ${className}`}
        >
            <div
                className="absolute inset-0 opacity-0 group-hover:opacity-100 transition-opacity duration-300"
                style={{
                    background: `radial-gradient(350px circle at var(--mouse-x) var(--mouse-y), rgba(79, 70, 229, 0.15), transparent 80%)`,
                }}
            />
            <div className={`absolute -inset-px z-0 opacity-20 group-hover:opacity-30 transition-opacity duration-300 ${bgClass}`} />

            <div className="relative z-10 flex flex-col items-start h-full">
                <div className="mb-4 p-4 bg-slate-800 border border-slate-700 rounded-lg transition-colors duration-300 group-hover:bg-slate-700/50">
                    {icon}
                </div>
                <h3 className="text-xl font-semibold text-slate-100">{title}</h3>
                <p className="mt-2 text-slate-400">{description}</p>
                {children}
            </div>
        </div>
    );
};


export default AuthPage;