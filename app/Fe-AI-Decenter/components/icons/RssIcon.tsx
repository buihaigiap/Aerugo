import React from "react";

export const RssIcon: React.FC<React.SVGProps<SVGSVGElement>> = (props) => (
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
      d="M12.75 19.5v-.75a7.5 7.5 0 00-7.5-7.5h-.75"
    />
    <path
      strokeLinecap="round"
      strokeLinejoin="round"
      d="M15 19.5a15 15 0 00-15-15"
    />
    <path
      strokeLinecap="round"
      strokeLinejoin="round"
      d="M19.5 4.5a19.5 19.5 0 00-19.5 19.5"
    />
    <circle cx="5.25" cy="19.5" r="2.25" fill="currentColor" />
  </svg>
);
