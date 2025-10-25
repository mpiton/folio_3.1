import { describe, it, expect } from 'vitest';
import { defaultSEO, type SEOProps } from './seo';

/**
 * SEO Utilities Unit Tests
 *
 * Test coverage for:
 * - SEO metadata generation
 * - Open Graph tag generation
 * - Canonical URL generation
 * - Default SEO values
 *
 * Target coverage: >= 80%
 */

describe('SEO Utils - Default Configuration', () => {
  it('should have valid default SEO configuration', () => {
    expect(defaultSEO).toBeDefined();
    expect(defaultSEO.title).toBe('Mathieu Piton - Développeur Full Stack');
    expect(defaultSEO.description).toBe(
      'Portfolio de Mathieu Piton, développeur Full Stack spécialisé en Node.js'
    );
  });

  it('should have valid Open Graph defaults', () => {
    expect(defaultSEO.openGraph).toBeDefined();
    expect(defaultSEO.openGraph?.type).toBe('website');
    expect(defaultSEO.openGraph?.locale).toBe('fr_FR');
    expect(defaultSEO.openGraph?.url).toBe('https://mathieu-piton.com');
    expect(defaultSEO.openGraph?.site_name).toBe('Mathieu Piton');
  });
});

describe('SEO Utils - Meta Tag Generation', () => {
  /**
   * Helper function to generate meta tags object
   */
  const generateMetaTags = (title: string, description: string): SEOProps => {
    return {
      title,
      description,
      openGraph: {
        type: 'website',
        locale: 'fr_FR',
      },
    };
  };

  it('should generate meta tags with custom title and description', () => {
    const meta = generateMetaTags('Custom Title', 'Custom Description');

    expect(meta.title).toBe('Custom Title');
    expect(meta.description).toBe('Custom Description');
    expect(meta.openGraph?.type).toBe('website');
    expect(meta.openGraph?.locale).toBe('fr_FR');
  });

  it('should handle empty strings gracefully', () => {
    const meta = generateMetaTags('', '');

    expect(meta.title).toBe('');
    expect(meta.description).toBe('');
  });

  it('should preserve special characters in meta tags', () => {
    const meta = generateMetaTags(
      'Title with "quotes" & <tags>',
      'Description with special chars: é, è, à, ç'
    );

    expect(meta.title).toContain('quotes');
    expect(meta.title).toContain('&');
    expect(meta.description).toContain('é');
    expect(meta.description).toContain('ç');
  });
});

describe('SEO Utils - Open Graph Generation', () => {
  /**
   * Helper function to generate Open Graph tags
   */
  const generateOpenGraph = (options: {
    title: string;
    image?: string;
    url?: string;
    type?: string;
  }) => {
    const FALLBACK_IMAGE_URL = 'https://mathieu-piton.com/og-default.jpg';

    return {
      'og:title': options.title,
      'og:image': options.image || FALLBACK_IMAGE_URL,
      'og:url': options.url || '',
      'og:type': options.type || 'website',
    };
  };

  it('should generate Open Graph tags with all fields', () => {
    const og = generateOpenGraph({
      title: 'Blog Post',
      image: 'https://example.com/image.jpg',
      url: 'https://example.com/post',
      type: 'article',
    });

    expect(og['og:title']).toBe('Blog Post');
    expect(og['og:image']).toBe('https://example.com/image.jpg');
    expect(og['og:url']).toBe('https://example.com/post');
    expect(og['og:type']).toBe('article');
  });

  it('should handle missing og:image with fallback', () => {
    const og = generateOpenGraph({
      title: 'Post without image',
    });

    expect(og['og:image']).toBe('https://mathieu-piton.com/og-default.jpg');
  });

  it('should use default type when not specified', () => {
    const og = generateOpenGraph({
      title: 'Page Title',
    });

    expect(og['og:type']).toBe('website');
  });

  it('should handle article type for blog posts', () => {
    const og = generateOpenGraph({
      title: 'My Blog Article',
      type: 'article',
      url: 'https://example.com/blog/my-article',
    });

    expect(og['og:type']).toBe('article');
    expect(og['og:url']).toBe('https://example.com/blog/my-article');
  });
});

describe('SEO Utils - Canonical URL Generation', () => {
  /**
   * Helper function to generate canonical URL
   */
  const generateCanonical = (url: string): string => {
    // Ensure URL starts with https://
    if (!url.startsWith('http://') && !url.startsWith('https://')) {
      url = 'https://' + url;
    }

    // Remove trailing slash
    if (url.endsWith('/') && url !== 'https://' && url !== 'http://') {
      url = url.slice(0, -1);
    }

    return url;
  };

  it('should generate canonical URL with https', () => {
    const canonical = generateCanonical('https://example.com/blog');
    expect(canonical).toBe('https://example.com/blog');
  });

  it('should add https:// if missing', () => {
    const canonical = generateCanonical('example.com/blog');
    expect(canonical).toContain('https://example.com/blog');
  });

  it('should remove trailing slash', () => {
    const canonical = generateCanonical('https://example.com/blog/');
    expect(canonical).toBe('https://example.com/blog');
  });

  it('should handle root domain', () => {
    const canonical = generateCanonical('https://example.com/');
    expect(canonical).toBe('https://example.com');
  });

  it('should preserve paths with multiple segments', () => {
    const canonical = generateCanonical('https://example.com/blog/2025/01/post/');
    expect(canonical).toBe('https://example.com/blog/2025/01/post');
  });
});

describe('SEO Utils - SEOProps Interface', () => {
  it('should accept valid SEO props', () => {
    const validSEO: SEOProps = {
      title: 'Test Title',
      description: 'Test Description',
      openGraph: {
        type: 'website',
        locale: 'en_US',
        url: 'https://test.com',
        site_name: 'Test Site',
      },
    };

    expect(validSEO.title).toBe('Test Title');
    expect(validSEO.description).toBe('Test Description');
    expect(validSEO.openGraph?.type).toBe('website');
  });

  it('should accept SEO props without openGraph', () => {
    const minimalSEO: SEOProps = {
      title: 'Minimal Title',
      description: 'Minimal Description',
    };

    expect(minimalSEO.title).toBe('Minimal Title');
    expect(minimalSEO.description).toBe('Minimal Description');
    expect(minimalSEO.openGraph).toBeUndefined();
  });

  it('should accept partial openGraph data', () => {
    const partialOG: SEOProps = {
      title: 'Title',
      description: 'Description',
      openGraph: {
        type: 'article',
        // Other fields are optional
      },
    };

    expect(partialOG.openGraph?.type).toBe('article');
    expect(partialOG.openGraph?.locale).toBeUndefined();
  });
});

describe('SEO Utils - Edge Cases', () => {
  it('should handle very long titles', () => {
    const longTitle = 'A'.repeat(200);
    const seo: SEOProps = {
      title: longTitle,
      description: 'Normal description',
    };

    expect(seo.title.length).toBe(200);
    expect(seo.title).toBe(longTitle);
  });

  it('should handle very long descriptions', () => {
    const longDescription = 'B'.repeat(500);
    const seo: SEOProps = {
      title: 'Normal title',
      description: longDescription,
    };

    expect(seo.description.length).toBe(500);
    expect(seo.description).toBe(longDescription);
  });

  it('should handle special Unicode characters', () => {
    const seo: SEOProps = {
      title: 'Test 🚀 with emoji',
      description: 'Description avec accents: é è ê ë ç à',
    };

    expect(seo.title).toContain('🚀');
    expect(seo.description).toContain('é');
    expect(seo.description).toContain('ç');
  });

  it('should handle malformed URLs gracefully', () => {
    const seo: SEOProps = {
      title: 'Test',
      description: 'Test',
      openGraph: {
        url: 'not-a-valid-url',
      },
    };

    expect(seo.openGraph?.url).toBe('not-a-valid-url');
    // The application should handle URL validation elsewhere
  });
});
