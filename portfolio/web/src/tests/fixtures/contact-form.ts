/**
 * Contact Form Test Fixtures
 *
 * Sample data for testing contact form functionality
 */

export interface ContactFormData {
  name: string;
  email: string;
  message: string;
}

/**
 * Valid contact form submission
 */
export const validContactForm: ContactFormData = {
  name: 'John Doe',
  email: 'john.doe@example.com',
  message: 'Hello, I would like to discuss a potential project collaboration.',
};

/**
 * Contact form with minimum required fields
 */
export const minimalContactForm: ContactFormData = {
  name: 'Jane',
  email: 'jane@test.com',
  message: 'Hi',
};

/**
 * Contact form with long message
 */
export const longMessageContactForm: ContactFormData = {
  name: 'Bob Smith',
  email: 'bob.smith@company.com',
  message: `Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
  Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.
  Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.
  Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.`,
};

/**
 * Invalid contact form data (for validation testing)
 */
export const invalidContactForms = {
  emptyName: {
    name: '',
    email: 'test@example.com',
    message: 'Test message',
  },
  invalidEmail: {
    name: 'Test User',
    email: 'invalid-email',
    message: 'Test message',
  },
  emptyMessage: {
    name: 'Test User',
    email: 'test@example.com',
    message: '',
  },
  missingFields: {
    name: '',
    email: '',
    message: '',
  },
};

/**
 * Contact form with special characters (for XSS/injection testing)
 */
export const specialCharactersContactForm: ContactFormData = {
  name: "O'Brien <script>alert('xss')</script>",
  email: 'test+special@example.com',
  message: 'Message with "quotes" and special chars: @#$%^&*()',
};

/**
 * Rate-limited contact form (triggers 429 response in mock)
 */
export const rateLimitedContactForm: ContactFormData = {
  name: 'Rate Limited User',
  email: 'ratelimited@test.com',
  message: 'This should trigger rate limiting',
};
