import z33;

database = z33.PYDB("database")

database.add_workflow("workflow1")
database.add_resource("workflow1", "resource1")
database.add_resource("workflow1", "resource2")
database.add_resource( "workflow1", "resource3")

database.add_container("workflow1", "container1")

database.add_store("workflow1","store1")

database.res_add_user_time("workflow1", "resource1", "user1", 1, 2.0)
database.res_add_user_time("workflow1", "resource2", "user2", 1, 2.0)
database.res_add_user_time("workflow1", "resource3", "user3", 1, 2.0)

database.res_add_enter_time

database.printdb()