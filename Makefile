CURRENT_DIR=$(PWD)

RDB: configRDB initRDB populateRDB metadataRDB

configRDB:
	@connection=$$(psql bibliotheque -c 'select current_date;'); \
	if [ "$$connection" = "" ]; then \
		createdb bibliotheque; \
		connection=$$(psql bibliotheque -c 'select current_date;'); \
		if [ "$$connection" = "" ]; then \
			echo "\n\nERROR : The configuration of RDB with PostgreSQL doesn't work."; \
			echo "Verify that the user '`whoami`' can connect to PostgreSQL and you could try to create the database by yourself."; \
		else \
			echo "The Relational Database 'bibliotheque' was successfully created."; \
		fi; \
	else \
		echo "The Relational Database 'bibliotheque' is ready."; \
	fi

initRDB:
	@echo "\nDrop and Create the tables :"
	@psql -d bibliotheque -c "\i $(CURRENT_DIR)/RDB/init.sql"
	@psql -d bibliotheque -c "\i $(CURRENT_DIR)/RDB/triggers.sql"
	@psql -d bibliotheque -c "\i $(CURRENT_DIR)/RDB/procedures.sql"
	
populateRDB:
	@echo "\nPopulate the database :"
	@cd RDB && chmod +x populate.sh && ./populate.sh

metadataRDB:
	@psql -d bibliotheque -t -A -F "" -f $(CURRENT_DIR)/RDB/meta_data.sql -o meta_data_RDB.json
	@echo "\nSuccessfully extract and save the meta data of the database."

GDB: initGDB

initGDB:
	@cypher-shell -a $(ADDRESS) -u $(USERNAME) -p '$(PASSWORD)' -f init.cql

help :
	@echo "Flags usages :"
	@echo
	@echo "RDB             : To execute all the commands that manage the relationnal database 'bibliotheque'."
	@echo "|-> configRDB   : To verify that 'psql' work and that '$(shell whoami)' can connect to PostgreSQL."
	@echo "|-> initRDB     : To drop/create the tables, create the procedures/function and the triggers."
	@echo "|-> populateRDB : To load the CSV data in the databse."
	@echo "|-> metadataRDB : To extract and save the metada of the database."