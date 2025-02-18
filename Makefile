CURRENT_DIR=$(PWD)

"": configPython demo

configPython:
	python3 manage_env.py

demo: PostgreSQL Neo4j

PostgreSQL: configPostgreSQL initPostgreSQL populatePostgreSQL metadataPostgreSQL

configPostgreSQL:
	@connection=$$(psql bibliotheque -c 'select current_date;'); \
	if [ "$$connection" = "" ]; then \
		createdb bibliotheque; \
		connection=$$(psql bibliotheque -c 'select current_date;'); \
		if [ "$$connection" = "" ]; then \
			echo "\n\nERROR : The configuration of PostgreSQL doesn't work."; \
			echo "Verify that the user '`whoami`' can connect to PostgreSQL and you could try to create the database by yourself."; \
		else \
			echo "The Relational Database 'bibliotheque' was successfully created."; \
		fi; \
	else \
		echo "The Relational Database 'bibliotheque' is ready."; \
	fi

initPostgreSQL:
	@echo "\nDrop and Create the tables :"
	@psql -d bibliotheque -c "\i $(CURRENT_DIR)/PostgreSQL/init.sql"
	@psql -d bibliotheque -c "\i $(CURRENT_DIR)/PostgreSQL/triggers.sql"
	@psql -d bibliotheque -c "\i $(CURRENT_DIR)/PostgreSQL/procedures.sql"
	
populatePostgreSQL:
	@echo "\nPopulate the database :"
	@cd PostgreSQL && chmod +x populate.sh && ./populate.sh ../Data/

metadataPostgreSQL:
	@# @psql -d bibliotheque -t -A -F "" -f $(CURRENT_DIR)/PostgreSQL/meta_data.sql -o meta_data_PostgreSQL.json
	@# @echo "\nSuccessfully extract and save the meta data of the database."

Neo4j: initNeo4j

initNeo4j:
	@# @echo "Example of command for Neo4jcypher-shell -a ADDRESS -u USERNAME -p 'PASSWORD' -f init.cql"

help :
	@echo "Flags usages :"
	@echo
	@echo "PostgreSQL             : To execute all the commands that manage the PostgreSQL database 'bibliotheque'."
	@echo "|-> configPostgreSQL   : To verify that 'psql' work and that '$(shell whoami)' can connect to PostgreSQL."
	@echo "|-> initPostgreSQL     : To drop/create the tables, create the procedures/function and the triggers."
	@echo "|-> populatePostgreSQL : To load the CSV data in the databse."
	@echo "|-> metadataPostgreSQL : To extract and save the metada of the database."
	@echo
	@echo "Neo4j         : To execute all the commands that manage the Neo4j database 'bibliotheque'."
	@echo "|-> initNeo4j : To drop and detach all the nodes."