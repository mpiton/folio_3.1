print('Starting MongoDB initialization...');

// Switch to admin database and authenticate as root
db = db.getSiblingDB('admin');
db.auth(process.env.MONGO_INITDB_ROOT_USERNAME, process.env.MONGO_INITDB_ROOT_PASSWORD);
print('Authenticated as root user');

// Create application user in portfolio database
db = db.getSiblingDB(process.env.MONGO_DB);
print('Switched to database:', process.env.MONGO_DB);

db.createUser({
    user: process.env.MONGO_USER,
    pwd: process.env.MONGO_PASSWORD,
    roles: [
        { role: "readWrite", db: process.env.MONGO_DB },
        { role: "dbAdmin", db: process.env.MONGO_DB }
    ]
});
print('Created application user:', process.env.MONGO_USER);

// Initialize portfolio database
db.createCollection('portfolio');
print('Created portfolio collection');

db.portfolio.createIndex({ "url": 1 });
db.portfolio.createIndex({ "pub_date": 1 }, { expireAfterSeconds: 7776000 }); // 90 days TTL
db.portfolio.createIndex({ "title": 1 });
db.portfolio.createIndex({ "url": 1, "pub_date": 1 }, { unique: true });
print('Created portfolio indexes');

db.createCollection('contacts');
print('Created contacts collection');

db.contacts.createIndex({ "email": 1 });
db.contacts.createIndex({ "created_at": 1 }, { expireAfterSeconds: 15552000 }); // 180 days TTL
db.contacts.createIndex({ "email": 1, "created_at": -1 });
print('Created contacts indexes');

// Initialize test database
db = db.getSiblingDB(process.env.MONGO_DB + "_test");
print('Switched to test database');

db.createCollection('portfolio');
print('Created portfolio collection in test database');

db.portfolio.createIndex({ "url": 1 });
db.portfolio.createIndex({ "pub_date": 1 }, { expireAfterSeconds: 7776000 }); // 90 days TTL
db.portfolio.createIndex({ "title": 1 });
db.portfolio.createIndex({ "url": 1, "pub_date": 1 }, { unique: true });
print('Created portfolio indexes in test database');

db.createCollection('contacts');
print('Created contacts collection in test database');

db.contacts.createIndex({ "email": 1 });
db.contacts.createIndex({ "created_at": 1 }, { expireAfterSeconds: 15552000 }); // 180 days TTL
db.contacts.createIndex({ "email": 1, "created_at": -1 });
print('Created contacts indexes in test database');

db.createUser({
    user: process.env.MONGO_USER,
    pwd: process.env.MONGO_PASSWORD,
    roles: [
        { role: "readWrite", db: process.env.MONGO_DB + "_test" },
        { role: "dbAdmin", db: process.env.MONGO_DB + "_test" }
    ]
});

// Initialisation de la base de test
const testDb = db.getSiblingDB('portfolio_test');

// Création et initialisation de la collection RSS
print('Creating portfolio collection...');
testDb.portfolio.drop();
testDb.createCollection('portfolio');

// Ajout d'articles de test
print('Adding test articles...');
const testArticles = [];
for (let i = 0; i < 20; i++) {
    testArticles.push({
        title: `Test Article ${i + 1}`,
        url: `https://example.com/article-${i + 1}`,
        pub_date: new Date(),
        description: `Test Description for article ${i + 1}`,
        image_url: 'https://placehold.co/600x400',
        created_at: new Date()
    });
}

const result = testDb.portfolio.insertMany(testArticles);
print(`Inserted ${result.insertedCount} test articles`);

// Création des index pour la collection portfolio
print('Creating portfolio indexes...');
testDb.portfolio.createIndex({ "url": 1, "pub_date": -1 });
testDb.portfolio.createIndex({ "pub_date": 1 }, { expireAfterSeconds: 604800 }); // 7 jours

// Création et initialisation de la collection contacts
print('Creating contacts collection...');
testDb.contacts.drop();
testDb.createCollection('contacts');

// Création des index pour la collection contacts
print('Creating contacts indexes...');
testDb.contacts.createIndex({ "email": 1, "created_at": -1 });
testDb.contacts.createIndex({ "created_at": 1 }, { expireAfterSeconds: 604800 }); // 7 jours

// Initialisation de la base RSS de test
const rssDb = db.getSiblingDB('rss_test');
print('Creating RSS test collection...');
rssDb.feeds_test.drop();
rssDb.createCollection('feeds_test');

// Ajout de flux RSS de test
print('Adding test feeds...');
const testFeeds = [];
for (let i = 0; i < 20; i++) {
    testFeeds.push({
        title: `Test Feed ${i + 1}`,
        url: `https://example.com/feed-${i + 1}`,
        pub_date: new Date(),
        description: `Test Feed Description ${i + 1}`,
        image_url: 'https://placehold.co/600x400',
        created_at: new Date()
    });
}

const feedResult = rssDb.feeds_test.insertMany(testFeeds);
print(`Inserted ${feedResult.insertedCount} test feeds`);

// Création des index pour la collection feeds_test
print('Creating RSS indexes...');
rssDb.feeds_test.createIndex({ "url": 1, "pub_date": -1 });
rssDb.feeds_test.createIndex({ "pub_date": 1 }, { expireAfterSeconds: 604800 }); // 7 jours

print('MongoDB initialization completed successfully');
