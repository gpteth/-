from flask_sqlalchemy import SQLAlchemy

db = SQLAlchemy()

def initialize_app(app):
    app.config['SQLALCHEMY_DATABASE_URI'] = 'sqlite:///influencerfi.db'  # Replace with your database URL
    db.init_app(app)

# Create the database tables
def create_tables():
    with app.app_context():
        db.create_all()

# Initialize the database and create tables
def initialize_database(app):
    initialize_app(app)
    create_tables()
