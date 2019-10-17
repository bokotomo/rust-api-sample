from orator import DatabaseManager, Model
from e2e.config import db_host, db_database, db_user, db_password


def __set_db():
    config = {
        'mysql': {
            'driver': 'mysql',
            'host': db_host,
            'database': db_database,
            'user': db_user,
            'password': db_password,
            'prefix': ''
        }
    }
    db = DatabaseManager(config)
    Model.set_connection_resolver(db)
    return db


db = __set_db()
