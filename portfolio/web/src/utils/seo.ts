export interface SEOProps {
    title: string;
    description: string;
    openGraph?: {
        type?: string;
        locale?: string;
        url?: string;
        site_name?: string;
    };
}

export const defaultSEO: SEOProps = {
    title: 'Mathieu Piton - Développeur Full Stack',
    description: 'Portfolio de Mathieu Piton, développeur Full Stack spécialisé en Node.js',
    openGraph: {
        type: 'website',
        locale: 'fr_FR',
        url: 'https://mathieu-piton.com',
        site_name: 'Mathieu Piton',
    },
};
