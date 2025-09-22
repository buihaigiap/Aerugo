
import React, { useEffect } from 'react';
import { Link, useLocation } from 'react-router-dom';
import CodeBlock from '../components/docs/CodeBlock';

const DocsPage: React.FC = () => {
    const navItems = [
        { href: '#introduction', label: 'Introduction' },
        { href: '#organizations', label: 'Managing Organizations' },
        { href: '#repositories', label: 'Managing Repositories' },
        { href: '#docker-usage', label: 'Using Docker' },
        { href: '#tos', label: 'Terms of Service' },
    ];

    const REGISTRY_HOST = 'registry.example.com'; // Placeholder, matches other components

    const location = useLocation();

    useEffect(() => {
        // Handle scrolling to the correct section when the hash in the URL changes.
        // This is needed for links from other pages (e.g., footer) to work correctly.
        if (location.hash) {
            const id = location.hash.substring(1);
            setTimeout(() => {
                const element = document.getElementById(id);
                if (element) {
                    element.scrollIntoView({
                        behavior: 'smooth',
                        block: 'start'
                    });
                }
            }, 100); // Small delay to ensure the page has rendered
        }
    }, [location.hash]);

    const handleNavClick = (e: React.MouseEvent<HTMLAnchorElement>, targetId: string) => {
        e.preventDefault();
        const id = targetId.substring(1);
        const element = document.getElementById(id);
        if (element) {
            element.scrollIntoView({
                behavior: 'smooth',
                block: 'start'
            });
            // Update the URL hash without reloading the page
            window.history.pushState(null, '', targetId);
        }
    };

    return (
        <div className="flex flex-col lg:flex-row gap-12">
            {/* Sidebar */}
            <aside className="lg:w-64 lg:flex-shrink-0 lg:sticky lg:top-24 self-start ml-6">
                <nav>
                    <h3 className="text-xl font-bold uppercase text-slate-400 tracking-wider mb-4">On this page</h3>
                    <ul className="space-y-2">
                        {navItems.map(item => (
                            <li key={item.href}>
                                <a
                                    href={item.href}
                                    onClick={(e) => handleNavClick(e, item.href)}
                                    className="block text-slate-300 hover:text-white transition-colors py-1"
                                >
                                    {item.label}
                                </a>
                            </li>
                        ))}
                    </ul>
                </nav>
            </aside>

            {/* Main content */}
            <main className="flex-1 min-w-0">
                <div className="space-y-16">
                    <section id="introduction" className="scroll-mt-20">
                        <h1 className="text-4xl font-extrabold tracking-tight text-white">User Guide</h1>
                        <p className="mt-4 text-xl text-slate-300 leading-relaxed">
                            Welcome to the Aerugo Registry! This guide will walk you through managing your container images using this web interface. Here you can create organizations for your teams, manage your repositories, and control access for your members.
                        </p>
                         <div className="mt-8">
                            <Link
                                to="/repositories"
                                className="inline-block px-8 py-3 bg-indigo-600 hover:bg-indigo-500 text-white font-semibold rounded-md shadow-lg transition-transform transform hover:scale-105 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 focus:ring-offset-slate-900"
                            >
                                Get Started
                            </Link>
                        </div>
                    </section>

                    <section id="organizations" className="scroll-mt-20">
                        <h2 className="text-3xl font-bold text-slate-100 border-b border-slate-700 pb-3">Managing Organizations</h2>
                        <div className="mt-6 space-y-6 text-slate-300 leading-relaxed">
                            <p>An organization acts as a workspace for your team. It contains repositories and members. You can create multiple organizations to separate projects or teams.</p>

                            <h3 className="text-2xl font-semibold text-slate-100 !mt-8">Creating an Organization</h3>
                            <ol className="list-decimal list-inside space-y-3 pl-5">
                                <li>Navigate to the <Link to="/organizations" className="font-medium text-indigo-400 hover:text-indigo-300">Organizations</Link> page from the main navigation.</li>
                                <li>Click the "Create New" button.</li>
                                <li>
                                    Fill in the form:
                                    <ul className="list-disc list-inside space-y-2 pl-5 mt-3">
                                        <li><strong className="font-semibold text-slate-100">Display Name:</strong> The name that will be shown throughout the UI (e.g., "My Awesome Team").</li>
                                        <li><strong className="font-semibold text-slate-100">Organization Name (URL):</strong> A unique, URL-friendly identifier for your organization. This is used in docker commands (e.g., "my-awesome-team"). It must be lowercase and can only contain letters, numbers, and hyphens.</li>
                                        <li>The other fields are optional and can be used to add more detail.</li>
                                    </ul>
                                </li>
                                <li>Click "Create Organization" to finish.</li>
                            </ol>

                            <h3 className="text-2xl font-semibold text-slate-100 !mt-8">Managing Members</h3>
                            <p>Once you've selected an organization, you can manage its members from the "Members" tab.</p>
                            <ul className="list-disc list-inside space-y-2 pl-5">
                                <li><strong className="font-semibold text-slate-100">Adding a Member:</strong> Click "Add Member", enter the user's registered email address, and assign them a role.</li>
                                <li><strong className="font-semibold text-slate-100">Changing Roles:</strong> You can change a member's role using the dropdown next to their name.</li>
                                <li><strong className="font-semibold text-slate-100">Removing a Member:</strong> Click the trash icon to remove a member from the organization.</li>
                            </ul>

                            <h3 className="text-2xl font-semibold text-slate-100 !mt-8">Member Roles</h3>
                             <ul className="list-disc list-inside space-y-2 pl-5">
                                <li><strong className="font-semibold text-slate-100">Owner:</strong> Has full administrative control over the organization, its repositories, and its members. Can delete the organization.</li>
                                <li><strong className="font-semibold text-slate-100">Admin:</strong> Can manage repositories and members (except other Owners).</li>
                                <li><strong className="font-semibold text-slate-100">Member:</strong> Can view and (depending on repository settings) push/pull images within the organization.</li>
                            </ul>
                        </div>
                    </section>

                    <section id="repositories" className="scroll-mt-20">
                        <h2 className="text-3xl font-bold text-slate-100 border-b border-slate-700 pb-3">Managing Repositories</h2>
                        <div className="mt-6 space-y-6 text-slate-300 leading-relaxed">
                            <p>A repository is a collection of related container images, identified by different tags (e.g., <code>:latest</code>, <code>:v1.0</code>). For example, you might have a repository named <code>my-app</code> to store all versions of your application's image.</p>

                            <h3 className="text-2xl font-semibold text-slate-100 !mt-8">Creating a Repository</h3>
                            <ol className="list-decimal list-inside space-y-3 pl-5">
                                <li>Navigate to the <Link to="/repositories" className="font-medium text-indigo-400 hover:text-indigo-300">Repositories</Link> page.</li>
                                <li>From the dropdown menu in the top-right, select the organization you want to create the repository in.</li>
                                <li>Click the "Create Repository" button.</li>
                                <li><strong className="font-semibold text-slate-100">Name</strong> your repository. This name will be part of the image URL.</li>
                                <li>Choose the repository's <strong className="font-semibold text-slate-100">Visibility</strong>.</li>
                            </ol>

                            <h3 className="text-2xl font-semibold text-slate-100 !mt-8">Public vs. Private Repositories</h3>
                            <ul className="list-disc list-inside space-y-2 pl-5">
                                <li><strong className="font-semibold text-slate-100">Public:</strong> Anyone, even unauthenticated users, can pull images from this repository.</li>
                                <li><strong className="font-semibold text-slate-100">Private:</strong> Only members of the organization can pull images. You control who has access.</li>
                            </ul>
                        </div>
                    </section>

                    <section id="docker-usage" className="scroll-mt-20">
                        <h2 className="text-3xl font-bold text-slate-100 border-b border-slate-700 pb-3">Using Docker</h2>
                        <div className="mt-6 space-y-6 text-slate-300 leading-relaxed">
                            <p>To push and pull images, you'll use the Docker command-line tool. The following commands show you how.</p>

                            <h3 className="text-2xl font-semibold text-slate-100 !mt-8">1. Log In to the Registry</h3>
                            <p>First, you need to log in with your Aerugo account credentials. This command only needs to be run once.</p>
                            <CodeBlock code={`docker login ${REGISTRY_HOST}`} language="bash" />

                            <h3 className="text-2xl font-semibold text-slate-100 !mt-8">2. Tag Your Image</h3>
                            <p>Before you can push a local image, you must tag it with the full registry path. The format is <code>{REGISTRY_HOST}/[organization-name]/[repository-name]:[tag]</code>.</p>
                            <CodeBlock code={`docker tag my-local-image:latest ${REGISTRY_HOST}/my-awesome-team/my-app:v1.0`} language="bash" />

                            <h3 className="text-2xl font-semibold text-slate-100 !mt-8">3. Push the Image</h3>
                            <p>Now, push the tagged image to the registry.</p>
                            <CodeBlock code={`docker push ${REGISTRY_HOST}/my-awesome-team/my-app:v1.0`} language="bash" />
                            <p>You should now see the new tag appear in the repository details within the UI.</p>

                            <h3 className="text-2xl font-semibold text-slate-100 !mt-8">4. Pull the Image</h3>
                            <p>To pull the image on another machine (or after removing it locally), use the same full path.</p>
                            <CodeBlock code={`docker pull ${REGISTRY_HOST}/my-awesome-team/my-app:v1.0`} language="bash" />
                        </div>
                    </section>

                    <section id="tos" className="scroll-mt-20">
                        <h2 className="text-3xl font-bold text-slate-100 border-b border-slate-700 pb-3">Terms of Service</h2>
                        <div className="mt-6 space-y-6 text-slate-300 leading-relaxed">
                            <p>Last Updated: {new Date().toLocaleDateString()}</p>
                            
                            <h3 className="text-2xl font-semibold text-slate-100 !mt-8">1. Acceptance of Terms</h3>
                            <p>By accessing or using the Aerugo Registry service ("Service"), you agree to be bound by these Terms of Service ("Terms"). If you disagree with any part of the terms, then you may not access the Service.</p>

                            <h3 className="text-2xl font-semibold text-slate-100 !mt-8">2. User Accounts</h3>
                            <p>You are responsible for safeguarding the password that you use to access the Service and for any activities or actions under your password. You agree not to disclose your password to any third party. You must notify us immediately upon becoming aware of any breach of security or unauthorized use of your account.</p>

                            <h3 className="text-2xl font-semibold text-slate-100 !mt-8">3. User Content</h3>
                            <p>You retain full ownership of any container images, data, or other content you upload to the Service ("Content"). By uploading Content, you grant us a worldwide, non-exclusive, royalty-free license to host, store, and distribute your Content solely for the purpose of providing and operating the Service. You are solely responsible for your Content and the consequences of storing and distributing it.</p>
                            <p>You represent and warrant that you have all necessary rights to your Content and that your Content does not infringe upon any third-party rights, contain any malware, or violate any applicable laws.</p>

                            <h3 className="text-2xl font-semibold text-slate-100 !mt-8">4. Acceptable Use</h3>
                            <p>You agree not to use the Service for any purpose that is illegal or prohibited by these Terms. You agree not to:</p>
                             <ul className="list-disc list-inside space-y-2 pl-5">
                                <li>Upload or distribute any Content that is unlawful, harmful, or infringes on the intellectual property rights of others.</li>
                                <li>Engage in any activity that interferes with or disrupts the Service (or the servers and networks which are connected to the Service).</li>
                                <li>Attempt to gain unauthorized access to any part of the Service, other accounts, or computer systems.</li>
                            </ul>

                            <h3 className="text-2xl font-semibold text-slate-100 !mt-8">5. Termination</h3>
                            <p>We may terminate or suspend your access to our Service immediately, without prior notice or liability, for any reason whatsoever, including without limitation if you breach the Terms. Upon termination, your right to use the Service will immediately cease. We reserve the right to delete your account and all associated Content upon termination.</p>

                            <h3 className="text-2xl font-semibold text-slate-100 !mt-8">6. Disclaimer of Warranties</h3>
                            <p>The Service is provided on an "AS IS" and "AS AVAILABLE" basis. We make no warranties, expressed or implied, regarding the reliability, security, or availability of the Service.</p>

                            <h3 className="text-2xl font-semibold text-slate-100 !mt-8">7. Limitation of Liability</h3>
                            <p>In no event shall Aerugo Registry, nor its directors, employees, partners, or agents, be liable for any indirect, incidental, special, consequential or punitive damages, including without limitation, loss of profits, data, use, goodwill, or other intangible losses, resulting from your access to or use of or inability to access or use the Service.</p>

                            <h3 className="text-2xl font-semibold text-slate-100 !mt-8">8. Changes to Terms</h3>
                            <p>We reserve the right, at our sole discretion, to modify or replace these Terms at any time. We will provide notice of any changes by posting the new Terms on this page. Your continued use of the Service after any such changes constitutes your acceptance of the new Terms.</p>
                        </div>
                    </section>
                </div>
            </main>
        </div>
    );
};

export default DocsPage;