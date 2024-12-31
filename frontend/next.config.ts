import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  reactStrictMode: true, // Enabling strict mode for better development experience
  images: {
    domains: ['ucarecdn.com'],
  },
  env: {
    NEXT_PUBLIC_CUSTOM_DOMAIN: 'https://example.com',
  },
  // Add any other configuration options you need
};

export default nextConfig;
