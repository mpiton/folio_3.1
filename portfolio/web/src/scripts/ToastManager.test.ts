import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest';
import { ToastManager, type ToastOptions } from './ToastManager';

/**
 * ToastManager Unit Tests
 *
 * Test coverage for:
 * - Singleton pattern
 * - Toast creation (success, error, warning, info)
 * - CSS injection and styling
 * - Auto-dismiss functionality
 * - Event listeners
 *
 * Target coverage: >= 85%
 */

describe('ToastManager - Singleton Pattern', () => {
  let instance1: ToastManager;
  let instance2: ToastManager;

  beforeEach(() => {
    // Reset DOM
    document.body.innerHTML = '';
    document.head.innerHTML = '';
    // Reset singleton instance using private static field
    // @ts-expect-error - Accessing private static field for testing
    ToastManager.instance = undefined;
  });

  afterEach(() => {
    vi.clearAllTimers();
    vi.useRealTimers();
  });

  it('1.1 - should return same instance', () => {
    instance1 = ToastManager.getInstance();
    instance2 = ToastManager.getInstance();

    expect(instance1).toBe(instance2);
  });

  it('1.2 - should initialize only once', () => {
    // Get multiple instances
    const instances = [
      ToastManager.getInstance(),
      ToastManager.getInstance(),
      ToastManager.getInstance(),
    ];

    // All should be the same instance
    expect(instances[0]).toBe(instances[1]);
    expect(instances[1]).toBe(instances[2]);
  });
});

describe('ToastManager - Creation', () => {
  let manager: ToastManager;

  beforeEach(() => {
    // Reset DOM
    document.body.innerHTML = '';
    document.head.innerHTML = '';
    // Reset singleton instance using private static field
    // @ts-expect-error - Accessing private static field for testing
    ToastManager.instance = undefined;

    manager = ToastManager.getInstance();
  });

  afterEach(() => {
    // Cleanup
  });

  it('2.1 - should create success toast', () => {
    const options: ToastOptions = {
      type: 'success',
      title: 'Success!',
      message: 'Operation completed successfully',
      duration: 0, // Disable auto-dismiss for testing
    };

    manager.show(options);

    // Verify toast container exists
    const container = document.getElementById('toastContainer');
    expect(container).toBeTruthy();

    // Verify toast element
    const toast = container?.querySelector('.toast');
    expect(toast).toBeTruthy();
    expect(toast?.getAttribute('data-type')).toBe('success');

    // Verify content
    const title = toast?.querySelector('.toast-title');
    const message = toast?.querySelector('.toast-message');
    expect(title?.textContent).toBe('Success!');
    expect(message?.textContent).toBe('Operation completed successfully');
  });

  it('2.2 - should create error toast', () => {
    const options: ToastOptions = {
      type: 'error',
      title: 'Error!',
      message: 'An error occurred',
      duration: 0,
    };

    manager.show(options);

    const container = document.getElementById('toastContainer');
    const toast = container?.querySelector('.toast');

    expect(toast?.getAttribute('data-type')).toBe('error');
    expect(toast?.querySelector('.toast-title')?.textContent).toBe('Error!');
    expect(toast?.querySelector('.toast-message')?.textContent).toBe('An error occurred');
  });

  it('2.3 - should create warning toast', () => {
    const options: ToastOptions = {
      type: 'warning',
      title: 'Warning!',
      message: 'Please check your input',
      duration: 0,
    };

    manager.show(options);

    const container = document.getElementById('toastContainer');
    const toast = container?.querySelector('.toast');

    expect(toast?.getAttribute('data-type')).toBe('warning');
    expect(toast?.querySelector('.toast-title')?.textContent).toBe('Warning!');
    expect(toast?.querySelector('.toast-message')?.textContent).toBe('Please check your input');
  });

  it('2.4 - should manage toast queue (multiple toasts visible)', () => {
    const toast1: ToastOptions = {
      type: 'success',
      title: 'Toast 1',
      message: 'First toast',
      duration: 0,
    };

    const toast2: ToastOptions = {
      type: 'error',
      title: 'Toast 2',
      message: 'Second toast',
      duration: 0,
    };

    const toast3: ToastOptions = {
      type: 'warning',
      title: 'Toast 3',
      message: 'Third toast',
      duration: 0,
    };

    // Create 3 toasts rapidly
    manager.show(toast1);
    manager.show(toast2);
    manager.show(toast3);

    // Verify all toasts are in the container
    const container = document.getElementById('toastContainer');
    const toasts = container?.querySelectorAll('.toast');

    expect(toasts?.length).toBe(3);

    // Verify each toast has correct type
    expect(toasts?.[0].getAttribute('data-type')).toBe('success');
    expect(toasts?.[1].getAttribute('data-type')).toBe('error');
    expect(toasts?.[2].getAttribute('data-type')).toBe('warning');
  });
});

describe('ToastManager - Styling', () => {
  let manager: ToastManager;

  beforeEach(() => {
    document.body.innerHTML = '';
    document.head.innerHTML = '';
    vi.useFakeTimers();
    // Reset singleton instance using private static field
    // @ts-expect-error - Accessing private static field for testing
    ToastManager.instance = undefined;

    manager = ToastManager.getInstance();
  });

  afterEach(() => {
    vi.clearAllTimers();
    vi.useRealTimers();
  });

  it('3.1 - should inject CSS on first show', () => {
    // Initially, no styles
    expect(document.getElementById('toast-styles')).toBeNull();

    const options: ToastOptions = {
      type: 'info',
      title: 'Info',
      message: 'Test message',
      duration: 0,
    };

    manager.show(options);

    // After show, style tag should exist
    const styleTag = document.getElementById('toast-styles');
    expect(styleTag).toBeTruthy();
    expect(styleTag?.tagName).toBe('STYLE');
    expect(styleTag?.textContent).toContain('.toast-container');
    expect(styleTag?.textContent).toContain('.toast');
  });

  it('3.2 - should not inject CSS twice', () => {
    const options: ToastOptions = {
      type: 'success',
      title: 'Test',
      message: 'Message',
      duration: 0,
    };

    // Show multiple toasts
    manager.show(options);
    manager.show(options);
    manager.show(options);

    // Should only have one style tag
    const styleTags = document.querySelectorAll('#toast-styles');
    expect(styleTags.length).toBe(1);
  });
});

describe('ToastManager - Auto-dismiss', () => {
  let manager: ToastManager;

  beforeEach(() => {
    document.body.innerHTML = '';
    document.head.innerHTML = '';
    vi.useFakeTimers();
    // Reset singleton instance using private static field
    // @ts-expect-error - Accessing private static field for testing
    ToastManager.instance = undefined;

    manager = ToastManager.getInstance();
  });

  afterEach(() => {
    vi.clearAllTimers();
    vi.useRealTimers();
  });

  it('4.1 - should auto-dismiss after 5s default', async () => {
    const options: ToastOptions = {
      type: 'success',
      title: 'Auto-dismiss test',
      message: 'This should disappear',
      // duration not specified, should use default 5000ms
    };

    manager.show(options);

    // Toast should be visible initially
    let container = document.getElementById('toastContainer');
    let toasts = container?.querySelectorAll('.toast');
    expect(toasts?.length).toBe(1);

    // Advance time by 5000ms (default duration)
    vi.advanceTimersByTime(5000);

    // Wait for transition and removal (300ms)
    vi.advanceTimersByTime(300);

    // Toast should be removed
    container = document.getElementById('toastContainer');
    toasts = container?.querySelectorAll('.toast');
    expect(toasts?.length).toBe(0);
  });

  it('4.2 - should respect custom duration', async () => {
    const customDuration = 2000;
    const options: ToastOptions = {
      type: 'info',
      title: 'Custom duration',
      message: 'Will dismiss in 2s',
      duration: customDuration,
    };

    manager.show(options);

    // Toast should be visible initially
    let container = document.getElementById('toastContainer');
    let toasts = container?.querySelectorAll('.toast');
    expect(toasts?.length).toBe(1);

    // Advance time by custom duration
    vi.advanceTimersByTime(customDuration);

    // Wait for transition
    vi.advanceTimersByTime(300);

    // Toast should be removed
    container = document.getElementById('toastContainer');
    toasts = container?.querySelectorAll('.toast');
    expect(toasts?.length).toBe(0);
  });

  it('4.3 - should dismiss on button click', () => {
    const options: ToastOptions = {
      type: 'warning',
      title: 'Click to dismiss',
      message: 'Test message',
      duration: 0, // Disable auto-dismiss
    };

    manager.show(options);

    // Toast should be visible
    let container = document.getElementById('toastContainer');
    let toasts = container?.querySelectorAll('.toast');
    expect(toasts?.length).toBe(1);

    // Find and click close button
    const closeButton = toasts?.[0].querySelector('.toast-close') as HTMLButtonElement;
    expect(closeButton).toBeTruthy();

    closeButton.click();

    // Wait for transition
    vi.advanceTimersByTime(300);

    // Toast should be removed
    container = document.getElementById('toastContainer');
    toasts = container?.querySelectorAll('.toast');
    expect(toasts?.length).toBe(0);
  });
});

describe('ToastManager - Events', () => {
  beforeEach(() => {
    document.body.innerHTML = '';
    document.head.innerHTML = '';
    vi.useFakeTimers();
    // Reset singleton instance using private static field
    // @ts-expect-error - Accessing private static field for testing
    ToastManager.instance = undefined;

    // Initialize singleton for events
    ToastManager.getInstance();
  });

  afterEach(() => {
    vi.clearAllTimers();
    vi.useRealTimers();
  });

  it('5.1 - should respond to showToast custom event', () => {
    const eventDetail: ToastOptions = {
      type: 'success',
      title: 'Event Toast',
      message: 'Triggered by custom event',
      duration: 0,
    };

    // Dispatch custom event
    const event = new CustomEvent('showToast', { detail: eventDetail });
    document.dispatchEvent(event);

    // Toast should appear
    const container = document.getElementById('toastContainer');
    const toast = container?.querySelector('.toast');

    expect(toast).toBeTruthy();
    expect(toast?.getAttribute('data-type')).toBe('success');
    expect(toast?.querySelector('.toast-title')?.textContent).toBe('Event Toast');
    expect(toast?.querySelector('.toast-message')?.textContent).toBe('Triggered by custom event');
  });
});
