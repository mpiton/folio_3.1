const DUMMY_DATA = [
  {
    title: "10 Astuces pour booster votre productivité en télétravail",
    link: "#",
    description: "Découvrez nos conseils pour rester efficace et motivé lorsque vous travaillez depuis chez vous.",
    pubDate: new Date("2023-06-10"),
    imageUrl: "https://via.placeholder.com/300x200?text=Article+1"
  },
  {
    title: "Les meilleures extensions VS Code pour les développeurs web",
    link: "#",
    description: "Nous avons sélectionné pour vous les extensions VS Code les plus utiles pour le développement web.",
    pubDate: new Date("2023-06-08"),
    imageUrl: "https://via.placeholder.com/300x200?text=Article+2"
  },
  {
    title: "Comment bien choisir sa base de données NoSQL ?",
    link: "#",
    description: "Critères à prendre en compte et comparatif des principales solutions du marché.",
    pubDate: new Date("2023-06-05"),
    imageUrl: "https://via.placeholder.com/300x200?text=Article+3"
  },
  {
    title: "Tutoriel : Créer une API REST avec Node.js et Express",
    link: "#",
    description: "Apprenez à développer une API robuste avec authentification JWT en suivant notre guide étape par étape.",
    pubDate: new Date("2023-06-02"),
    imageUrl: "https://via.placeholder.com/300x200?text=Article+4"
  }
];

function displayRssFeeds() {
  const rssContainer = document.querySelector('.rss-container');

  DUMMY_DATA.forEach(item => {
    const rssCard = document.createElement('div');
    rssCard.classList.add('rss-card');
    rssCard.innerHTML = `
      <img src="${item.imageUrl}" alt="${item.title}">
      <div class="rss-card-content">
        <h3>${item.title}</h3>
        <p>${item.pubDate.toLocaleDateString()}</p>
        <a href="${item.link}" target="_blank">Lire l'article</a>
      </div>
    `;
    rssContainer.appendChild(rssCard);
  });
}

displayRssFeeds();
