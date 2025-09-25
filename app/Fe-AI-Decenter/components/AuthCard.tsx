import React, { useRef, useEffect } from 'react';

interface AuthCardProps {
    title: string;
    subtitle: string;
    children: React.ReactNode;
}

const AuthCard: React.FC<AuthCardProps> = ({ title, subtitle, children }) => {
    const cardRef = useRef<HTMLDivElement>(null);

    useEffect(() => {
        const card = cardRef.current;
        if (!card) return;

        const handleMouseMove = (e: MouseEvent) => {
            const rect = card.getBoundingClientRect();
            const x = e.clientX - rect.left;
            const y = e.clientY - rect.top;
            card.style.setProperty('--mouse-x', `${x}px`);
            card.style.setProperty('--mouse-y', `${y}px`);
        };

        card.addEventListener('mousemove', handleMouseMove);

        return () => {
            card.removeEventListener('mousemove', handleMouseMove);
        };
    }, []);

    return (
        <div
            ref={cardRef}
            className="relative bg-slate-800/60 backdrop-blur-lg border border-slate-700/80 rounded-xl shadow-2xl shadow-indigo-950/50 animate-fade-in-up p-px overflow-hidden"
        >
            <div className="auth-card-animated-border" />
            <div className="relative w-full h-full bg-slate-800/95 rounded-xl p-8">
                <div className="absolute inset-0 auth-card-spotlight" />
                <div className="relative z-10">
                     <h2 className="text-3xl font-bold text-center text-slate-50 mb-2">
                        {title}
                    </h2>
                    <p className="text-center text-slate-400 mb-8">
                        {subtitle}
                    </p>
                    {children}
                </div>
            </div>
        </div>
    );
};

export default AuthCard;