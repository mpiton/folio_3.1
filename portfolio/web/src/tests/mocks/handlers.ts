import { http, HttpResponse } from 'msw';

// Base API URL - can be configured via env variable
const API_BASE_URL = import.meta.env.PUBLIC_API_URL || 'http://localhost:4010';

/**
 * MSW handlers for mocking API endpoints in tests
 *
 * These handlers intercept network requests during testing
 * and return mock responses without hitting the real API.
 */
export const handlers = [
  // POST /api/contact - Mock contact form submission
  http.post(`${API_BASE_URL}/api/contact`, async ({ request }) => {
    const body = (await request.json()) as Record<string, unknown>;

    // Simulate validation failure for invalid email
    if (body.email === 'invalid@test') {
      return HttpResponse.json({ error: 'Invalid email address' }, { status: 400 });
    }

    // Simulate rate limiting
    if (body.email === 'ratelimited@test.com') {
      return HttpResponse.json(
        { error: 'Rate limit exceeded. Please try again later.' },
        { status: 429 }
      );
    }

    // Success response
    return HttpResponse.json(
      {
        success: true,
        message: 'Message sent successfully. I will get back to you soon!',
      },
      { status: 200 }
    );
  }),

  // GET /api/rss - Mock RSS feed fetching
  http.get(`${API_BASE_URL}/api/rss`, () => {
    return HttpResponse.json(
      {
        items: [
          {
            id: 'mock-rss-1',
            title: 'Test Article 1',
            description: 'This is a test article description',
            link: 'https://example.com/article-1',
            pub_date: '2025-10-24T12:00:00Z',
            source: 'Test Blog',
            source_url: 'https://example.com/feed.xml',
          },
          {
            id: 'mock-rss-2',
            title: 'Test Article 2',
            description: 'Another test article description',
            link: 'https://example.com/article-2',
            pub_date: '2025-10-23T12:00:00Z',
            source: 'Test Blog',
            source_url: 'https://example.com/feed.xml',
          },
        ],
      },
      { status: 200 }
    );
  }),

  // GET /api/health - Mock health check endpoint
  http.get(`${API_BASE_URL}/api/health`, () => {
    return HttpResponse.json(
      { status: 'ok', timestamp: new Date().toISOString() },
      { status: 200 }
    );
  }),
];
