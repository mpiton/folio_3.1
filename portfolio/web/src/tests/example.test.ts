import { describe, it, expect } from 'vitest';
import { setupDOM, cleanupDOM, waitFor } from './fixtures/utils';
import { validContactForm } from './fixtures/contact-form';
import { contactApiResponses } from './fixtures/api-responses';

/**
 * Example Test Suite
 *
 * This is a simple example to verify the test infrastructure is working correctly.
 * This file can be removed once real tests are implemented.
 */

describe('Test Infrastructure', () => {
  it('should run basic assertions', () => {
    expect(true).toBe(true);
    expect(1 + 1).toBe(2);
  });

  it('should have access to DOM environment', () => {
    expect(document).toBeDefined();
    expect(window).toBeDefined();
  });

  it('should be able to create and manipulate DOM elements', () => {
    const container = setupDOM();
    expect(container).toBeDefined();
    expect(container.id).toBe('test-container');
    expect(document.getElementById('test-container')).toBe(container);

    cleanupDOM();
    expect(document.getElementById('test-container')).toBeNull();
  });

  it('should have access to test fixtures', () => {
    expect(validContactForm).toBeDefined();
    expect(validContactForm.name).toBe('John Doe');
    expect(validContactForm.email).toBe('john.doe@example.com');
  });

  it('should have access to API response fixtures', () => {
    expect(contactApiResponses.success).toBeDefined();
    expect(contactApiResponses.success.success).toBe(true);
  });

  it('should support async/await operations', async () => {
    const startTime = Date.now();
    await waitFor(100);
    const endTime = Date.now();

    expect(endTime - startTime).toBeGreaterThanOrEqual(100);
  });
});

describe('MSW API Mocking', () => {
  it('should intercept contact form API calls', async () => {
    const response = await fetch('http://localhost:4010/api/contact', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(validContactForm),
    });

    expect(response.ok).toBe(true);
    const data = await response.json();
    expect(data.success).toBe(true);
  });

  it('should handle rate limiting', async () => {
    const response = await fetch('http://localhost:4010/api/contact', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        name: 'Test',
        email: 'ratelimited@test.com',
        message: 'Test',
      }),
    });

    expect(response.status).toBe(429);
    const data = await response.json();
    expect(data.error).toContain('Rate limit exceeded');
  });

  it('should intercept RSS feed API calls', async () => {
    const response = await fetch('http://localhost:4010/api/rss');

    expect(response.ok).toBe(true);
    const data = await response.json();
    expect(data.items).toBeDefined();
    expect(Array.isArray(data.items)).toBe(true);
    expect(data.items.length).toBeGreaterThan(0);
  });
});
