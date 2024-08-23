# wordnet2db

Take Princeton's WordNet 3.1 files and outputs an SQLite database, SQL statements for importing, or a JSON file containing English words and definitions.

Files available from Princeton University at https://wordnet.princeton.edu/download/current-version

## Options
-c, --char-counts [<CHAR_COUNTS>...]  
&emsp;&emsp;Comma seperated list of character counts to save e.g. 4,7  
-d, --directory <DIRECTORY>  
&emsp;&emsp;Directory where WordNet files are located (index.adj, index.adv, data.adj, etc)  
-S, --dump-sql  
&emsp;&emsp;Renders database as SQL statements rather than an SQLite database  
-k, --keep-numbers  
&emsp;&emsp;Keep words with numbers  
-M, --max-chars <MAX_CHARS>  
&emsp;&emsp;Maximum character count of a word to save [default: 45]  
-m, --min-chars <MIN_CHARS>  
&emsp;&emsp;Minimum character count of a word to save [default: 0]  
-W, --only-whole-words  
&emsp;&emsp;Only keep words without punctuation or spaces  
-o, --output-directory <OUTPUT_DIRECTORY>  
&emsp;&emsp;Directory to place output file into (default: working directory)  
-J, --to-json  
&emsp;&emsp;Renders dictionary as JSON rather than an SQLite database  
-h, --help  
&emsp;&emsp;Print help (see more with '--help')  
-V, --version  
&emsp;&emsp;Print version  
