from neo4j import GraphDatabase, RoutingControl
import neo4j
import json
import os
import sys
import subprocess
import psycopg2

class PostgreSQL:

    def __init__(self, host:str, port:str, username:str, password:str, database:str):
        self.connection = (host, port, username, password)
        self.driver = None
        self.database = database

    def open(self):
        host, port, username, password = self.connection
        connection = psycopg2.connect(
            database=self.database,
            host=host,
            user=username,
            password=password,
            port=port
        )
        self.driver = connection.cursor()

    def execute_query(self, query:str):
        try:
            self.driver.execute(query)
            result = self.driver.fetchall()
            print(f"Result of the query :\n{result}") ; return result
        except psycopg2.ProgrammingError as error:
            if f"{error}" == "no results to fetch":
                print("There isn't data to fetch but your request was successfully executed.")
            else:
                print(f"ERROR : {error}\nPostgreSQL query executed :\n{query}")
                raise
        except Exception as error:
            print(f"ERROR : {error}\nPostgreSQL query executed :\n{query}")
            raise
    
    def execute_script(self, script_path:str):
        with open(script_path, "r") as fs:
            query = fs.read()
        self.execute_query(query)
    
    def export_meta_data(self, script_path:str, save_path:str="postgresql_meta_data.json"):
        with open(script_path, "r") as fs:
            query = fs.read()

        result = self.execute_query(query)[0][0]
        with open(save_path, "w") as fd:
            json.dump(result,fd)
    
    def export_tables(self, script_path:str, sql_function_name:str, save_path:str):
        """CAUTION : The ***script_path*** needs to be a path to a file that contains the definition of a POSTGRESQL function who return a JSON object that contains the tables.\n
        The ***sql_function_name*** is the name of the function create in the script : ***script_path***"""
        
        with open(script_path, "r") as fs:
            query_create_function = fs.read()

        self.execute_query(query_create_function)
    
        host, port, username, password = self.connection
        command = f"export PGPASSWORD='{password}' && psql -U {username} -h {host} -p {port} -d {self.database} -c '\copy (select {sql_function_name}()) to {save_path}'"
        
        try:
            result = subprocess.run(command, shell=True, check=True, text=True, capture_output=True, executable='/bin/bash')
            print(f"Result :\n{result.stdout}")
        except Exception as error:
            print(f"Error: {error}")
            raise

class Neo4j:

    def __init__(self, uri, username, password, database=None):
        self.connection = (uri, username, password)
        self.driver = None
        self.database = database

    def open(self):
        """Open the connection with the Database."""
        uri, username, password = self.connection
        self.driver = GraphDatabase.driver(uri, auth=(username, password))

    def close(self):
        """Close the connection with the Database."""
        self.driver.close()

    def execute_query(self, query:str):
        """**CAUTION** : You could run multiple queries but you must return to the line between each queries."""
        try:
            with self.driver.session() as session:
                result = session.run(
                    "CALL apoc.cypher.runMany($script, {statements: 'individual'}) YIELD result",
                    script=query
                )
                count = 1 ; print(f"\n==== Request n°{count} ====")
                for record in result :
                    if "rows" in record["result"].keys():
                        print(record["result"]) ; count+=1
                        print(f"\n==== Request n°{count} ====")
                    else:
                        print(record["result"])
                print("\033[F\033[K", end='')
        except neo4j.exceptions.TransientError as error:
            print(f"The following error could be caused by a script that match an delete the same nodes.\n{error}")
        except Exception as error:
            print(f"Error when runing the following query :\n{query}\n\n{error}")
            raise

    def execute_script(self, script_path:str):
        """**CAUTION** : You must return to the line between each queries."""
        with open(script_path, "r") as fs:
            query = fs.read()
        self.execute_query(query)
    
    def load_from_CSV(self, file_path:str, separator:str):
        """This function only create the nodes contained in the file.\n
        CAUTION : That function don't care about types, all the properties are Strings."""
        LABEL = os.path.basename(file_path)[:-4].upper()

        with open(file_path, "r") as fs:
            line = fs.readline()
            properties = line[:-1].split(separator)

            for line in fs :
                query = f"create (:{LABEL}" ; query += " {"
                values = line[:-1].split(separator)
                for index, key in enumerate(properties):
                    query += f"{key}: '{values[index]}',"
                query = query[:-1] ; query += "});"
                self.execute_query(query)
        print(f"\nSuccessfully load the data from '{file_path}'.")

    def load_from_postgresql(
        self, db_postgresql:PostgreSQL, meta_data_script:str, save1_path:str,
        export_tables_script:str, sql_function_name:str, save2_path:str
        ):
        """The argument of this method corresponding of the arguments needed by the following methods :
        \n-> **PostgreSQL.export_meta_data(** ```meta_data_script```, ```save_path``` **)**
        \n-> **PostgreSQL.export_tables(** ```export_tables_script```, ```export_query``` **)**"""
        
        db_postgresql.open()
        db_postgresql.export_meta_data(meta_data_script, save1_path)
        db_postgresql.export_tables(export_tables_script, sql_function_name, save2_path) # il faut récupérer le chemin dans l'export_query

        with open(save1_path, "r") as fs:
            CONSTRAINTS = json.load(fs)
        
        for table in CONSTRAINTS["tables"]:
            LABEL = table["table_name"].upper()
            function_name = ""

            FOREIGN_KEYS = []

            # Create constraints to translate these concepts : PrimaryKey / Isn't NULL
            for column in table["columns"]:
                if column["primary_key"] != None:
                    self.execute_query(f"create constraint unique_{LABEL.lower()}_{column['column_name']} if not exists for (n:{LABEL})\
                        require n.{column['column_name']} is unique;")
            """
            PRIMARY_KEYS = set(pf for pf in table["primary_keys"])
            FOREIGN_KEYS = []
            FOREIGN_COLUMN = set()
            if table["foreign_keys"] != None:
                for fk in table["foreign_keys"]: FOREIGN_KEYS.append(fk); FOREIGN_COLUMN.add(fk["column_name"])
            
            # Create constraints to translate these concepts : PrimaryKey / Isn't NULL
            for column in table["columns"]:
                if column["column_name"] in FOREIGN_COLUMN:
                    continue 
                else:
                    if column["column_name"] in PRIMARY_KEYS:
                        self.execute_query(f"create constraint unique_{LABEL.lower()}_{column['column_name']} if not exists for (n:{LABEL})\
                            require n.{column['column_name']} is unique;")
                    if column["is_nullable"] == "NO":
                        self.execute_query(f"create constraint nonull_{LABEL.lower()}_{column['column_name']} if not exists for (n:{LABEL})\
                            require n.{column['column_name']} is not null;")
            """
            
if __name__ == "__main__":

    db_neo4j = Neo4j("neo4j://localhost:7687", "neo4j", "LUGOL2656", "neo4j")
    db_postgresql = PostgreSQL("127.0.0.1","5432","lugolbis","LUGOL2656","bibliotheque")

    def demo():
        # Demonstration of the code
        db_postgresql.open()
        script1_path = "/home/lugolbis/Bureau/UVSQ/L3_Info/S6/IN608/PostgreSQL/meta_data.sql"
        script2_path = "/home/lugolbis/Bureau/UVSQ/L3_Info/S6/IN608/PostgreSQL/export_tables.sql"

        save1_path = "/home/lugolbis/Bureau/UVSQ/L3_Info/S6/IN608/Neo4j/postgresql_meta_data.json"
        save2_path = "/home/lugolbis/Bureau/UVSQ/L3_Info/S6/IN608/Neo4j/postgresql_tables.json"

        function_name="export_db_to_json"

        db_postgresql.export_meta_data(script1_path,save1_path)
        db_postgresql.export_tables(script2_path,function_name,save2_path)

        return
        #db_neo4j.open()
        #db_neo4j.reset()
        #db_neo4j.close()
        db_neo4j.open()
        db_neo4j.execute_query("match (n:Product) return n;\n match (n) detach delete n;")
        #db_neo4j.load_from_postgresql(db_postgresql, script1_path, save1_path, script2_path, function_name, save2_path)
        db_neo4j.close()

    demo()