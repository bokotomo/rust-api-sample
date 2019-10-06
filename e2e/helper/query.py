from pypika import Query, Table, Field


def desiger_index():
    customers = Table('customers')
    q = Query.from_(customers).select('id', 'name')
    print(q.get_sql())


def desiger_add(id=1, name=""):
    customers = Table('customers')
    q = Query.into(customers).columns('id', 'name').insert(id, name)
    print(q.get_sql())
