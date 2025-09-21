import React from 'react';

export const RocketLaunchIcon: React.FC<React.SVGProps<SVGSVGElement>> = (props) => (
  <svg 
    xmlns="http://www.w3.org/2000/svg" 
    fill="none" 
    viewBox="0 0 24 24" 
    strokeWidth={1.5} 
    stroke="currentColor" 
    {...props}
  >
    <path 
        strokeLinecap="round" 
        strokeLinejoin="round" 
        d="M15.59 14.37a6 6 0 01-5.84 7.38v-4.82m5.84-2.56a12.025 12.025 0 01-4.132 4.91m5.84-2.56a12.01 12.01 0 00-4.132-4.91m0 0a6 6 0 01-7.38-5.84m7.38 5.84c-1.282-.536-2.222-1.728-2.56-3.135A6 6 0 0111.63 6.5m-5.84 7.38v4.82A6 6 0 016.5 11.63m0 0c.536-1.282 1.728-2.222 3.135-2.56m-3.135 2.56a12.01 12.01 0 004.91 4.132m-4.91-4.132a12.025 12.025 0 004.91-4.132" 
    />
  </svg>
);
