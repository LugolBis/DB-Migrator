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

    def convert_PostgreSQL_type(type:str):
        """Convert PostgreSQL Type into Neo4j type.\n
        CAUTION : TIMESTAMP -> DATETIME ; MONEY -> FLOAT"""
        type = type.upper()
        match type:
            case "INTEGER" | "BIGINT" | "SERIAL" : return "INTEGER"
            case "REAL" | "DOUBLE" | "PRECISION" : return "FLOAT"
            case "NUMERIC" | "DECIMAL" : return "FLOAT"
            case "VARCHAR" | "TEXT" | "CHAR" | "CHARACTER VARYING" : return "STRING"
            case "BOOLEAN" : return "BOOLEAN"
            case "DATE" : return "DATE"
            case "TIME" : return "LOCALTIME"
            case "TIMESTAMP" : return "DATETIME"
            case "INTERVAL" : return "DURATION"
            case "JSON" | "JSONB" : return "MAP"
            case "UUID" : return "STRING"
            case "ARRAY" : return "LIST"
            case "BYTEA" : return "BYTES"
            case "ENUM" : return "STRING"
            case "POINT" | "POLYGON" : return "STRING"
            case "CIDR" | "INET" : return "STRING"
            case "MONEY" : return "FLOAT"
            case "XML" : return "STRING" 
            case _:print(f"ERROR : Try to convert an unknow PostgreSQL type into a Neo4j\
                type.\nType in input : {type}"); raise
    
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

    def load_with_admin(self,import_dir_path:str):
        """CAUTION : This method need some configurations and specific version of Neo4j.\n
        This function use the input path to import the data withe the following command :\n
        ../bin/neo4j-admin database import full <database> --nodes=..."""
        pass

    def create_CSV_headers(self,metadata_file_path:str):
        with open(metadata_file_path, "r") as fs:
            CONSTRAINTS = json.load(fs)
        
        for table in CONSTRAINTS["tables"]:
            LABEL = table["table_name"].upper()
            HEADERS = f":ID;"
            FOREIGN_KEYS = {} # Key : Table target ; value : Header

            # Create constraints to translate these concepts : PrimaryKey / Isn't NULL
            for column in table["columns"]:
                function_name = f"{LABEL.lower()}_{column['column_name']}"
                if column["primary_key"] != None:
                    self.execute_query(f"create constraint unique_{function_name} if not exists for (n:{LABEL})\
                        require n.{column['column_name']} is unique;")
                if column["is_nullable"] == "NO":
                        self.execute_query(f"create constraint nonull_{function_name} if not exists for (n:{LABEL})\
                            require n.{column['column_name']} is not null;")
                
                # apoc.trigger.add() ???
                
                # Configure the HEADERS for the CSV file
                fk = column["foreign_key"]
                if fk == None:
                    HEADERS += f"{column['column_name']}:{Neo4j.convert_PostgreSQL_type(column['data_type'])};"
                else:
                    key = (fk[0]["referenced_table"].upper())
                    if key in FOREIGN_KEYS.keys():
                        FOREIGN_KEYS[key].append[(fk[0]["constraint_name"][:-5],fk[0]["referenced_column"])]
                    else:
                        FOREIGN_KEYS[key] = [(fk[0]["constraint_name"][:-5],fk[0]["referenced_column"])]

            HEADERS += ":LABEL"

            with open(f"{LABEL}.csv","w") as fd:
                fd.write(HEADERS)

            if FOREIGN_KEYS != {}:
                for key, val in FOREIGN_KEYS.items():
                    HEADERS_FK = ":START_ID;" ; COLUMN_FK = ""
                    for tuple in val:
                        HEADERS_FK += f"{tuple[1]};"
                        COLUMN_FK += f";{tuple[0]}"
                    HEADERS_FK = HEADERS_FK + ":END_ID;:TYPE" + COLUMN_FK
                    with open(f"{LABEL}_REF_{key}.csv","w") as fd:
                        fd.write(HEADERS_FK)

    def extract_nodes(tables_path:str):
        """Take in input the path of the JSON file that contains the tables of the postgresql content."""
        with open(tables_path,"r") as fs:
            TABLES = json.load(fs)

        for table, lines in TABLES.items():
            LABEL = table.upper()
            with open(f"{LABEL}.csv", "r") as fs:
                HEADERS = fs.read().split(";")
            with open(f"{LABEL}.csv", "a") as fd:
                counter = 0
                for line in lines:
                    new_line = f"\n{LABEL}{counter};"
                    for property in HEADERS[1:-1]:
                        new_line += f"{line[property.split(':')[0]]};"
                    new_line += LABEL
                    fd.write(new_line)
                    counter += 1

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

            # Générer un fichier JSON contenant les HEADERS des Label et de leurs relations
            # Pour cela on va regrouper les foreign keys par table référencée dans un dictionnaire
            # Afin de regrouper les colonnes -> properties d'une relation allant de la table A vers la table B           

        self.create_CSV_headers(save1_path)
        Neo4j.extract_nodes(save2_path)
            
if __name__ == "__main__":

    db_neo4j = Neo4j("neo4j://localhost:7687", "userA", "password", "userTest")
    db_postgresql = PostgreSQL("127.0.0.1","5432","lugolbis","LUGOL2656","bibliotheque")

    def demo():
        # Demonstration of the code
        db_postgresql.open()
        script1_path = "/home/lugolbis/Bureau/UVSQ/L3_Info/S6/IN608/PostgreSQL/meta_data.sql"
        script2_path = "/home/lugolbis/Bureau/UVSQ/L3_Info/S6/IN608/PostgreSQL/export_tables.sql"

        save1_path = "/home/lugolbis/Bureau/UVSQ/L3_Info/S6/IN608/Neo4j/postgresql_meta_data.json"
        save2_path = "/home/lugolbis/Bureau/UVSQ/L3_Info/S6/IN608/Neo4j/postgresql_tables.json"

        function_name="export_db_to_json"

        #db_postgresql.export_meta_data(script1_path,save1_path)
        #db_postgresql.export_tables(script2_path,function_name,save2_path)

        db_neo4j.open()
        db_neo4j.load_from_postgresql(db_postgresql, script1_path, save1_path, script2_path, function_name, save2_path)
        db_neo4j.close()

    demo()