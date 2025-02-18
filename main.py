from neo4j import GraphDatabase, RoutingControl
from neo4j.exceptions import DriverError, Neo4jError
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
    
    def export_tables(self, script_path:str, query_export:str):
        """CAUTION : The ***script_path*** needs to be a path to a file that contains the definition of a POSTGRESQL function who return a JSON object that contains the tables.\n
        ***query_export*** needs to have the following syntax : '\copy (select *your_select_function()*) to */your/path.json*'"""
        
        with open(script_path, "r") as fs:
            query_create_function = fs.read()

        self.execute_query(query_create_function)
    
        host, port, username, password = self.connection
        command = f"export PGPASSWORD='{password}' && psql -U {username} -h {host} -p {port} -d {self.database} -c '{query_export}'"
        
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
        """Open the connection withe the Database."""
        uri, username, password = self.connection
        self.driver = GraphDatabase.driver(uri, auth=(username, password))

    def close(self):
        """Close the connection with the Database."""
        self.driver.close()

    def execute_query(self, query:str):
        with self.driver.session() as session:
            try:
                result = self.driver.execute_query(
                    query,
                    database_=self.database
                )
                print(f"Result of the query :\n{result}") ; return result
            except (DriverError, Neo4jError) as error:
                print(f"ERROR : {error}\n\nCypher query executed :\n{query}")
                raise
    
    def execute_script(self, script_path:str):
        with open(script_path, "r") as fs:
            query = fs.read()
        self.execute_query(query)
    
    def load_from_CSV(self, file_path:str, separator:str):
        """This function only create the nodes contained in the file.\n
        CAUTION : That function don't care about types, all the properties are Strings."""
        LABEL = os.path.basename(file_path)[:-4]
        LABEL = LABEL[0].upper() + LABEL[1:]

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

    def load_from_postgresql(self, db_postgresql:PostgreSQL):
        pass

if __name__ == "__main__":
    scheme = "neo4j"  
    host_name = "localhost"
    port = 7687
    uri = f"{scheme}://{host_name}:{port}"
    username = "neo4j"
    password = "LUGOL2656"
    database = "neo4j"
    db_graph = Neo4j(uri, username, password, database)

    pH = "127.0.0.1"
    pP = "5432"
    pU = "lugolbis"
    pPW = "LUGOL2656"
    pD = "bibliotheque"
    db_postgresql = PostgreSQL(pH,pP,pU,pPW,pD)

    def demo():
        pass