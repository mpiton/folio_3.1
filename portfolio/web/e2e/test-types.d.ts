// Type definitions for E2E tests

declare global {
  interface Window {
    import?: {
      meta: {
        env: {
          PUBLIC_API_URL: string;
          MODE?: string;
        };
      };
    };
  }
}

export {};
