import React from 'react';
import { ArrowDownIcon } from './ArrowDownIcon';

const DiagramBox: React.FC<{ title: string; children?: React.ReactNode; className?: string }> = ({ title, children, className }) => (
    <div className={`bg-slate-800 border border-slate-600 rounded-md p-3 text-center flex flex-col justify-center ${className}`}>
        <h4 className="font-bold text-sm text-slate-100 m-0">{title}</h4>
        {children && <p className="text-xs text-slate-400 m-0 mt-1">{children}</p>}
    </div>
);

const ArchitectureDiagram: React.FC = () => {
    return (
        <div className="not-prose my-8 p-6 bg-slate-900/50 border border-slate-700 rounded-lg flex flex-col items-center gap-6 text-slate-300">
            <DiagramBox title="Docker Client / Admin Client" className="w-full max-w-sm" />
            <ArrowDownIcon className="w-6 h-6 text-slate-500" />
            <DiagramBox title="Load Balancer" children="HTTPS (Registry & Mgmt API)" className="w-full max-w-sm" />
            <ArrowDownIcon className="w-6 h-6 text-slate-500" />
            
            {/* Nodes tier */}
            <div className="flex flex-col sm:flex-row justify-center gap-4 w-full">
                <DiagramBox title="Aerugo Node" children="(Rust)" className="flex-1" />
                <DiagramBox title="Aerugo Node" children="(Rust)" className="flex-1" />
                <DiagramBox title="Aerugo Node" children="(Rust)" className="flex-1" />
            </div>

            <ArrowDownIcon className="w-6 h-6 text-slate-500" />

            {/* Backend Services Tier */}
            <div className="w-full p-4 border border-slate-700 border-dashed rounded-lg">
                <div className="flex flex-col md:flex-row justify-center items-stretch gap-4">
                    <DiagramBox title="Metadata Store" children="(e.g., PostgreSQL)" className="flex-1" />
                    <DiagramBox title="Cache Layer" children="(e.g., Redis)" className="flex-1" />
                    <DiagramBox title="S3-Compatible Object Storage" className="flex-1" />
                </div>
                <p className="text-xs text-center text-slate-500 mt-3">All nodes connect to these backend services</p>
            </div>
        </div>
    );
};

export default ArchitectureDiagram;