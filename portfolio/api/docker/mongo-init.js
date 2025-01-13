print('Starting MongoDB initialization script...');

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

print('MongoDB initialization completed successfully');
