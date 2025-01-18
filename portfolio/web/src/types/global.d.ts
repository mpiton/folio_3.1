declare global {
  interface Window {
    ToastManager: {
      new (toast: HTMLElement): {
        show(): void;
        close(): void;
        pause(): void;
        resume(): void;
        destroy(): void;
      };
    };
  }
}

export {};
