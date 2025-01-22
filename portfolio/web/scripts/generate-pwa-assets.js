import { promises as fs } from 'fs';
import sharp from 'sharp';
import path from 'path';

const ICONS_SIZES = [192, 512];
const SOURCE_ICON = 'public/favicon.svg';
const OUTPUT_DIR = 'public';

async function generatePWAIcons() {
  try {
    // Lire l'icône source
    const sourceBuffer = await fs.readFile(SOURCE_ICON);

    // Générer les icônes pour chaque taille
    for (const size of ICONS_SIZES) {
      await sharp(sourceBuffer)
        .resize(size, size)
        .toFile(path.join(OUTPUT_DIR, `icon-${size}x${size}.png`));

      console.log(`✓ Généré icon-${size}x${size}.png`);
    }

    // Générer le manifest.webmanifest
    const manifest = {
      name: 'Mathieu Piton - Portfolio',
      short_name: 'MP Portfolio',
      description: 'Portfolio de Mathieu Piton, développeur Full Stack',
      theme_color: '#578E7E',
      background_color: '#F5ECD5',
      display: 'standalone',
      start_url: '/',
      scope: '/',
      icons: ICONS_SIZES.map(size => ({
        src: `/icon-${size}x${size}.png`,
        sizes: `${size}x${size}`,
        type: 'image/png',
        ...(size === 512 ? { purpose: 'any maskable' } : {})
      }))
    };

    await fs.writeFile(
      path.join(OUTPUT_DIR, 'manifest.webmanifest'),
      JSON.stringify(manifest, null, 2)
    );
    console.log('✓ Généré manifest.webmanifest');

  } catch (error) {
    console.error('Erreur lors de la génération des assets PWA:', error);
    process.exit(1);
  }
}

generatePWAIcons();
