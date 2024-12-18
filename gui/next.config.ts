import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  /* config options here */
  output: 'export',
  distDir: '../target/bundled/dist/'
};

export default nextConfig;
