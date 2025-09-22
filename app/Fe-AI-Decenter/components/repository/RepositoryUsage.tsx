import React from 'react';
import { ChartBarIcon } from '../icons/ChartBarIcon';

const StatCard: React.FC<{ title: string; value: string }> = ({ title, value }) => (
    <div className="bg-slate-800/50 p-4 rounded-lg border border-slate-700">
        <h4 className="text-sm font-medium text-slate-400">{title}</h4>
        <p className="text-2xl font-bold text-slate-50 mt-1">{value}</p>
    </div>
);

const RepositoryUsage: React.FC = () => {
    return (
        <div className="space-y-8">
            <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
                <StatCard title="Total Pulls (30d)" value="1,428" />
                <StatCard title="Total Tags" value="3" />
                <StatCard title="Storage Used" value="381 MB" />
            </div>

            <div className="bg-slate-800/50 p-6 rounded-lg border border-slate-700">
                <h3 className="text-lg font-semibold text-slate-200 mb-4">Pulls in the Last 30 Days</h3>
                <div className="h-64 flex items-center justify-center">
                    <div className="text-center text-slate-500">
                        <ChartBarIcon className="w-16 h-16 mx-auto" />
                        <p className="mt-4">Chart data is not yet available.</p>
                    </div>
                </div>
            </div>
        </div>
    );
};

export default RepositoryUsage;
