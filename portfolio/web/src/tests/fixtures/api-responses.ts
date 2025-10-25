/**
 * API Response Test Fixtures
 *
 * Mock API responses for testing
 */

/**
 * Contact API Responses
 */
export const contactApiResponses = {
  success: {
    success: true,
    message: 'Message sent successfully. I will get back to you soon!',
  },
  validationError: {
    error: 'Invalid email address',
  },
  rateLimitError: {
    error: 'Rate limit exceeded. Please try again later.',
  },
  serverError: {
    error: 'Internal server error. Please try again later.',
  },
  networkError: {
    error: 'Network error. Please check your connection.',
  },
};

/**
 * RSS Feed Item Interface
 */
export interface RSSItem {
  id: string;
  title: string;
  description: string;
  link: string;
  pub_date: string;
  source: string;
  source_url: string;
}

/**
 * RSS API Responses
 */
export const rssApiResponses = {
  success: {
    items: [
      {
        id: 'rss-item-1',
        title: 'Introduction to Rust Programming',
        description: 'Learn the basics of Rust programming language and its key features.',
        link: 'https://example.com/rust-intro',
        pub_date: '2025-10-24T10:00:00Z',
        source: 'Tech Blog',
        source_url: 'https://example.com/feed.xml',
      },
      {
        id: 'rss-item-2',
        title: 'Building Web Apps with Astro',
        description: 'A comprehensive guide to building fast websites with Astro framework.',
        link: 'https://example.com/astro-guide',
        pub_date: '2025-10-23T14:30:00Z',
        source: 'Web Dev Daily',
        source_url: 'https://example.com/webdev-feed.xml',
      },
      {
        id: 'rss-item-3',
        title: 'TypeScript Best Practices',
        description: 'Essential TypeScript patterns and practices for production code.',
        link: 'https://example.com/typescript-best-practices',
        pub_date: '2025-10-22T09:15:00Z',
        source: 'Tech Blog',
        source_url: 'https://example.com/feed.xml',
      },
    ],
  },
  empty: {
    items: [],
  },
  error: {
    error: 'Failed to fetch RSS feeds',
  },
};

/**
 * Health Check API Response
 */
export const healthApiResponse = {
  status: 'ok',
  timestamp: '2025-10-24T12:00:00Z',
};

/**
 * Common HTTP Error Responses
 */
export const httpErrorResponses = {
  badRequest: {
    status: 400,
    error: 'Bad Request',
    message: 'Invalid request parameters',
  },
  unauthorized: {
    status: 401,
    error: 'Unauthorized',
    message: 'Authentication required',
  },
  forbidden: {
    status: 403,
    error: 'Forbidden',
    message: 'Access denied',
  },
  notFound: {
    status: 404,
    error: 'Not Found',
    message: 'Resource not found',
  },
  rateLimited: {
    status: 429,
    error: 'Too Many Requests',
    message: 'Rate limit exceeded',
  },
  serverError: {
    status: 500,
    error: 'Internal Server Error',
    message: 'An unexpected error occurred',
  },
  serviceUnavailable: {
    status: 503,
    error: 'Service Unavailable',
    message: 'Service temporarily unavailable',
  },
};
