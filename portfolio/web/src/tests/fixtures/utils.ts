/**
 * Test Utilities
 *
 * Helper functions for testing setup, rendering, and cleanup
 */

/**
 * Wait for a specified amount of time (for testing async operations)
 */
export const waitFor = (ms: number): Promise<void> => {
  return new Promise(resolve => setTimeout(resolve, ms));
};

/**
 * Create a mock HTMLElement for testing
 */
export const createMockElement = (
  tag: string,
  attributes: Record<string, string> = {}
): HTMLElement => {
  const element = document.createElement(tag);
  Object.entries(attributes).forEach(([key, value]) => {
    element.setAttribute(key, value);
  });
  return element;
};

/**
 * Simulate form submission event
 */
export const simulateFormSubmit = (form: HTMLFormElement): void => {
  const event = new Event('submit', {
    bubbles: true,
    cancelable: true,
  });
  form.dispatchEvent(event);
};

/**
 * Simulate input change event
 */
export const simulateInputChange = (input: HTMLInputElement, value: string): void => {
  input.value = value;
  const event = new Event('input', {
    bubbles: true,
    cancelable: true,
  });
  input.dispatchEvent(event);
};

/**
 * Simulate button click event
 */
export const simulateClick = (element: HTMLElement): void => {
  const event = new MouseEvent('click', {
    bubbles: true,
    cancelable: true,
  });
  element.dispatchEvent(event);
};

/**
 * Query DOM elements with type safety
 */
export const querySelector = <T extends HTMLElement>(
  selector: string,
  parent: Document | HTMLElement = document
): T | null => {
  return parent.querySelector<T>(selector);
};

export const querySelectorAll = <T extends HTMLElement>(
  selector: string,
  parent: Document | HTMLElement = document
): NodeListOf<T> => {
  return parent.querySelectorAll<T>(selector);
};

/**
 * Setup and cleanup DOM for tests
 */
export const setupDOM = (): HTMLElement => {
  const container = document.createElement('div');
  container.id = 'test-container';
  document.body.appendChild(container);
  return container;
};

export const cleanupDOM = (): void => {
  const container = document.getElementById('test-container');
  if (container) {
    container.remove();
  }
  // Clean up any remaining elements
  document.body.innerHTML = '';
};

/**
 * Mock fetch response helper
 */
export const createMockResponse = <T>(data: T, status = 200, ok = true): Response => {
  return {
    ok,
    status,
    json: async () => data,
    text: async () => JSON.stringify(data),
    headers: new Headers(),
    redirected: false,
    statusText: ok ? 'OK' : 'Error',
    type: 'basic',
    url: '',
    clone: function () {
      return this;
    },
    body: null,
    bodyUsed: false,
    arrayBuffer: async () => new ArrayBuffer(0),
    blob: async () => new Blob(),
    formData: async () => new FormData(),
  } as Response;
};

/**
 * Wait for an element to appear in the DOM
 */
export const waitForElement = async (selector: string, timeout = 3000): Promise<HTMLElement> => {
  const startTime = Date.now();

  while (Date.now() - startTime < timeout) {
    const element = document.querySelector<HTMLElement>(selector);
    if (element) {
      return element;
    }
    await waitFor(50);
  }

  throw new Error(`Element "${selector}" not found within ${timeout}ms`);
};

/**
 * Check if element has specific class
 */
export const hasClass = (element: HTMLElement, className: string): boolean => {
  return element.classList.contains(className);
};

/**
 * Get element text content safely
 */
export const getTextContent = (element: HTMLElement | null): string => {
  return element?.textContent?.trim() || '';
};

/**
 * Mock localStorage for tests
 */
export const mockLocalStorage = (() => {
  let store: Record<string, string> = {};

  return {
    getItem: (key: string) => store[key] || null,
    setItem: (key: string, value: string) => {
      store[key] = value.toString();
    },
    removeItem: (key: string) => {
      delete store[key];
    },
    clear: () => {
      store = {};
    },
    get length() {
      return Object.keys(store).length;
    },
    key: (index: number) => {
      const keys = Object.keys(store);
      return keys[index] || null;
    },
  };
})();

/**
 * Setup and teardown localStorage mock
 */
export const setupLocalStorageMock = (): void => {
  Object.defineProperty(window, 'localStorage', {
    value: mockLocalStorage,
    writable: true,
  });
};

export const cleanupLocalStorageMock = (): void => {
  mockLocalStorage.clear();
};
