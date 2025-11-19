/** @type {import('next').NextConfig} */
const nextConfig = {
  // Enable React Strict Mode
  reactStrictMode: true,

  // TypeScript configuration
  typescript: {
    // Enable type checking in development
    ignoreBuildErrors: false,
  },

  // ESLint configuration
  eslint: {
    // Warning: This allows production builds to successfully complete even if
    // your project has ESLint errors.
    ignoreDuringBuilds: true,
  },

  // Output configuration
  output: "standalone",

  // Webpack configuration
  webpack: (config, { isServer }) => {
    // Add any custom webpack configuration here
    if (!isServer) {
      // Resolve fallbacks for client-side
      config.resolve.fallback = {
        ...config.resolve.fallback,
        fs: false,
        net: false,
        tls: false,
        crypto: require.resolve("crypto-browserify"),
        stream: require.resolve("stream-browserify"),
        http: require.resolve("stream-http"),
        https: require.resolve("https-browserify"),
        os: require.resolve("os-browserify/browser"),
        path: require.resolve("path-browserify"),
      };
    }
    return config;
  },

  // Environment variables
  env: {
    // Add any environment variables here
  },
};

module.exports = nextConfig;
